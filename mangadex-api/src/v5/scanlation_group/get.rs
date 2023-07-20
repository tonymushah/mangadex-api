//! Builder for the scanlation group view endpoint.
//!
//! <https://api.mangadex.org/swagger.html#/ScanlationGroup/get-group-id>
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
//! let group_id = Uuid::new_v4();
//! let group_res = client
//!     .scanlation_group()
//!     .view()
//!     .group_id(&group_id)
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("group view: {:?}", group_res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;
use mangadex_api_schema::v5::GroupResponse;
use mangadex_api_types::ReferenceExpansionResource;

#[derive(Debug, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), pattern = "owned")]
pub struct GetGroup<'a> {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    pub(crate) http_client: HttpClientRef,

    #[serde(skip)]
    pub group_id: &'a Uuid,

    #[builder(setter(each = "include"), default)]
    pub includes: Vec<ReferenceExpansionResource>,
}

endpoint! {
    GET ("/group/{}", group_id),
    #[query] GetGroup<'_>,
    #[flatten_result] GroupResponse
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use time::OffsetDateTime;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::MangaDexDateTime;

    #[tokio::test]
    async fn get_group_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let group_id = Uuid::new_v4();

        let datetime = MangaDexDateTime::new(&OffsetDateTime::now_utc());

        let response_body = json!({
            "result": "ok",
            "response": "entity",
            "data": {
                "id": group_id,
                "type": "scanlation_group",
                "attributes": {
                    "name": "Scanlation Group",
                    "altNames": [
                        {
                            "en": "Alternative Name"
                        }
                    ],
                    "website": "https://example.org",
                    "ircServer": null,
                    "ircChannel": null,
                    "discord": null,
                    "contactEmail": null,
                    "description": null,
                    "twitter": null,
                    "focusedLanguages": ["en"],
                    "locked": false,
                    "official": false,
                    "verified": false,
                    "inactive": false,
                    "publishDelay": "P6WT5M",
                    "version": 1,
                    "createdAt": datetime.to_string(),
                    "updatedAt": datetime.to_string(),
                },
                "relationships": []
            }
        });

        Mock::given(method("GET"))
            .and(path_regex(r"/group/[0-9a-fA-F-]+"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _ = mangadex_client
            .scanlation_group()
            .get()
            .group_id(&group_id)
            .build()?
            .send()
            .await?;

        Ok(())
    }
}
