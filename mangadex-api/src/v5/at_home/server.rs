//! Builder for the MangaDex@Home node URL endpoint.
//!
//! <https://api.mangadex.org/swagger.html#/AtHome/get-at-home-server-chapterId>
//!
//! # Examples
//!
//! ```rust
//! use uuid::Uuid;
//!
//! use mangadex_api::v5::MangaDexClient;
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! let chapter_id = Uuid::new_v4();
//! let node_url_res = client
//!     .at_home()
//!     .server()
//!     .chapter_id(&chapter_id)
//!     .force_port_443(true)
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("Node URL: {:?}", node_url_res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;
use mangadex_api_schema::v5::AtHomeServerResponse;

#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
#[derive(Debug, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), pattern = "owned", build_fn(error = "mangadex_api_types::error::BuilderError"))]
pub struct GetAtHomeServer {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub(crate) http_client: HttpClientRef,

    #[serde(skip_serializing)]
    pub chapter_id: Uuid,

    /// Force selecting from MangaDex@Home servers that use the standard HTTPS port 443.
    ///
    /// While the conventional port for HTTPS traffic is 443 and servers are encouraged to use it,
    /// it is not a hard requirement as it technically isn't anything special.
    ///
    /// However, some misbehaving school/office network will at time block traffic to non-standard
    /// ports, and setting this flag to true will ensure selection of a server that uses these.
    #[builder(default)]
    pub force_port_443: bool,
}

endpoint! {
    GET ("/at-home/server/{}", chapter_id),
    #[query] GetAtHomeServer,
    #[flatten_result] AtHomeServerResponse
}

#[cfg(test)]
mod tests {
    use fake::faker::internet::en::Password;
    use fake::Fake;
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::{HttpClient, MangaDexClient};

    #[tokio::test]
    async fn activate_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let chapter_id = Uuid::new_v4();
        let hash: String = Password(16..24).fake();

        let response_body = json!({
            "result": "ok",
            "baseUrl": "https://example.org",
            "chapter": {
                "hash": hash,
                "data": [
                    "1.jpg"
                ],
                "dataSaver": [
                    "1.jpg"
                ],
            }
        });

        Mock::given(method("GET"))
            .and(path_regex(r"/at-home/server/[0-9a-fA-F-]+"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .at_home()
            .server()
            .chapter_id(chapter_id)
            .force_port_443(true)
            .build()?
            .send()
            .await?;

        assert_eq!(res.base_url, Url::parse("https://example.org")?);
        assert_eq!(res.chapter.hash, hash);
        assert_eq!(res.chapter.data, vec!["1.jpg"]);
        assert_eq!(res.chapter.data_saver, vec!["1.jpg"]);

        Ok(())
    }
}
