//! Builder for the cover art list endpoint.
//!
//! <https://api.mangadex.org/docs/swagger.html#/Cover/get-cover>
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
//! let cover_id = Uuid::new_v4();
//! let cover_res = client
//!     .cover()
//!     .get()
//!     .add_cover_id(&cover_id)
//!     .send()
//!     .await?;
//!
//! println!("covers: {:?}", cover_res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use mangadex_api_schema::v5::CoverCollection;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;
use mangadex_api_types::{CoverSortOrder, Language, ReferenceExpansionResource};

/// Query parameters for `/cover`.
#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
#[derive(Debug, Serialize, Clone, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(
    setter(into, strip_option),
    default,
    build_fn(error = "crate::error::BuilderError")
)]
#[non_exhaustive]
pub struct ListCover {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub http_client: HttpClientRef,

    pub limit: Option<u32>,
    pub offset: Option<u32>,
    #[serde(rename = "manga")]
    #[builder(setter(each = "add_manga_id"))]
    pub manga_ids: Vec<Uuid>,
    #[serde(rename = "ids")]
    #[builder(setter(each = "add_cover_id"))]
    pub cover_ids: Vec<Uuid>,
    #[serde(rename = "uploaders")]
    #[builder(setter(each = "add_uploader_id"))]
    pub uploader_ids: Vec<Uuid>,
    #[builder(setter(each = "locale"))]
    pub locales: Vec<Language>,
    pub order: Option<CoverSortOrder>,
    #[builder(setter(each = "include"))]
    pub includes: Vec<ReferenceExpansionResource>,
}

endpoint! {
    GET "/cover",
    #[query] ListCover,
    #[flatten_result] crate::Result<CoverCollection>,
    ListCoverBuilder
}

#[cfg(test)]
mod tests {
    use fake::faker::lorem::en::Sentence;
    use fake::Fake;
    use serde_json::json;
    use time::OffsetDateTime;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::error::Error;
    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::{Language, MangaDexDateTime, ResponseType};

    #[tokio::test]
    async fn list_cover_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let cover_id = Uuid::new_v4();
        let description: String = Sentence(1..3).fake();

        let datetime = MangaDexDateTime::new(&OffsetDateTime::now_utc());

        let response_body = json!({
            "result": "ok",
            "response": "collection",
            "data": [
                {
                    "id": cover_id,
                    "type": "cover_art",
                    "attributes": {
                        "volume": "1",
                        "fileName": "1.jpg",
                        "description": description,
                        "locale": "en",
                        "version": 1,
                        "createdAt": datetime.to_string(),
                        "updatedAt": datetime.to_string(),

                    },
                    "relationships": []
                }
            ],
            "limit": 1,
            "offset": 0,
            "total": 1
        });

        Mock::given(method("GET"))
            .and(path("/cover"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client.cover().get().limit(1u32).send().await?;

        assert_eq!(res.response, ResponseType::Collection);
        let cover = &res.data[0];
        assert_eq!(cover.id, cover_id);
        assert_eq!(cover.attributes.volume, Some("1".to_string()));
        assert_eq!(cover.attributes.file_name, "1.jpg".to_string());
        assert_eq!(cover.attributes.description, description);
        assert_eq!(cover.attributes.locale, Some(Language::English));
        assert_eq!(cover.attributes.version, 1);
        assert_eq!(
            cover.attributes.created_at.to_string(),
            datetime.to_string()
        );
        assert_eq!(
            cover.attributes.updated_at.as_ref().unwrap().to_string(),
            datetime.to_string()
        );

        Ok(())
    }

    #[tokio::test]
    async fn list_cover_handles_400() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let error_id = Uuid::new_v4();

        let response_body = json!({
            "result": "error",
            "errors": [{
                "id": error_id.to_string(),
                "status": 400,
                "title": "Invalid limit",
                "detail": "Limit must be between 1 and 100"
            }]
        });

        Mock::given(method("GET"))
            .and(path("/cover"))
            .respond_with(ResponseTemplate::new(400).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .cover()
            .get()
            .limit(0u32)
            .send()
            .await
            .expect_err("expected error");

        if let Error::Api(errors) = res {
            assert_eq!(errors.errors.len(), 1);

            assert_eq!(errors.errors[0].id, error_id);
            assert_eq!(errors.errors[0].status, 400);
            assert_eq!(errors.errors[0].title, Some("Invalid limit".to_string()));
            assert_eq!(
                errors.errors[0].detail,
                Some("Limit must be between 1 and 100".to_string())
            );
        }

        Ok(())
    }
}
