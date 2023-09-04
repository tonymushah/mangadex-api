//! Builder for the manga reading status endpoint.
//!
//! <https://api.mangadex.org/swagger.html#/Manga/get-manga-id-status>
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
//! let manga_id = Uuid::new_v4();
//! let res = client
//!     .manga()
//!     .reading_status()
//!     .manga_id(&manga_id)
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("status: {:?}", res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;
use mangadex_api_schema::v5::MangaReadingStatusResponse;

#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
#[derive(Debug, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), pattern = "owned", build_fn(error = "mangadex_api_types::error::BuilderError"))]
pub struct MangaReadingStatus {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub(crate) http_client: HttpClientRef,

    #[serde(skip_serializing)]
    pub manga_id: Uuid,
}

endpoint! {
    GET ("/manga/{}/status", manga_id),
    #[no_data auth] MangaReadingStatus,
    #[flatten_result] MangaReadingStatusResponse
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};

    #[tokio::test]
    async fn manga_reading_status_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let manga_id = Uuid::new_v4();
        let response_body = json!({
            "result": "ok",
            "status": "reading"
        });

        Mock::given(method("GET"))
            .and(path_regex(r"/manga/[0-9a-fA-F-]+/status"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _ = mangadex_client
            .manga()
            .reading_status()
            .manga_id(manga_id)
            .build()?
            .send()
            .await?;

        Ok(())
    }
}
