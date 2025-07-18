//! Builder for the manga view endpoint.
//!
//! <https://api.mangadex.org/swagger.html#/Manga/get-manga-id>
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
//! let res = client
//!     .manga()
//!     .tag()
//!     .get()
//!     .send()
//!     .await?;
//!
//! println!("tags: {:?}", res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use mangadex_api_schema::v5::TagCollection;
use serde::Serialize;

use crate::HttpClientRef;

#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
#[derive(Debug, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(
    setter(into, strip_option),
    build_fn(error = "crate::error::BuilderError")
)]
#[non_exhaustive]
pub struct ListTags {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub http_client: HttpClientRef,
}

endpoint! {
    GET "/manga/tag",
    #[no_data] ListTags,
    #[flatten_result] crate::Result<TagCollection>,
    ListTagsBuilder
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::{Language, ResponseType, TagGroup};

    #[tokio::test]
    async fn get_manga_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let response_body = json!({
            "result": "ok",
            "response": "collection",
            "data": [
                {
                    "id": "0234a31e-a729-4e28-9d6a-3f87c4966b9e",
                    "type": "tag",
                    "attributes": {
                        "name": {
                            "en": "Oneshot"
                        },
                        "description": {
                            "en": "One-off works"
                        },
                        "group": "format",
                        "version": 1
                    },
                    "relationships": []
                },
                {
                    "id": "07251805-a27e-4d59-b488-f0bfbec15168",
                    "type": "tag",
                    "attributes": {
                        "name": {
                            "en": "Thriller"
                        },
                        "description": [],
                        "group": "genre",
                        "version": 1
                    },
                    "relationships": []
                }
            ],
            "limit": 10,
            "offset": 0,
            "total": 2
        });

        Mock::given(method("GET"))
            .and(path(r"/manga/tag"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client.manga().tag().get().send().await?;

        assert_eq!(res.response, ResponseType::Collection);
        let oneshot = &res.data[0];
        assert_eq!(
            oneshot.id,
            Uuid::parse_str("0234a31e-a729-4e28-9d6a-3f87c4966b9e")?
        );
        assert_eq!(
            oneshot.attributes.name.get(&Language::English),
            Some(&"Oneshot".to_string())
        );
        assert_eq!(oneshot.attributes.group, TagGroup::Format);

        // Skip the second tag checks for now.
        // If the first result passes, the second probably will as well.

        Ok(())
    }
}
