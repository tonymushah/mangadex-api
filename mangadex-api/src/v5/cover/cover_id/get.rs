//! Builder for the cover view endpoint.
//!
//! <https://api.mangadex.org/docs/swagger.html#/Cover/get-cover-id>
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
//!     .cover_id(cover_id)
//!     .get()
//!     .send()
//!     .await?;
//!
//! println!("cover: {:?}", cover_res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;
use mangadex_api_schema::v5::CoverResponse;
use mangadex_api_types::ReferenceExpansionResource;

#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
#[derive(Debug, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(
    setter(into, strip_option),
    build_fn(error = "mangadex_api_types::error::BuilderError")
)]
pub struct GetCover {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub(crate) http_client: HttpClientRef,

    /// Manga **or** Cover ID.
    #[serde(skip_serializing)]
    pub cover_id: Uuid,

    #[builder(setter(each = "include"), default)]
    pub includes: Vec<ReferenceExpansionResource>,
}

endpoint! {
    GET ("/cover/{}", cover_id),
    #[query] GetCover,
    #[flatten_result] CoverResponse,
    GetCoverBuilder
}

#[cfg(test)]
mod tests {
    use fake::faker::lorem::en::Sentence;
    use fake::Fake;
    use serde_json::json;
    use time::OffsetDateTime;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::error::Error;
    use mangadex_api_types::{Language, MangaDexDateTime};

    #[tokio::test]
    async fn get_cover_fires_a_request_to_base_url() -> anyhow::Result<()> {
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
            "response": "entity",
            "data": {
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
        });

        Mock::given(method("GET"))
            .and(path_regex(r"/cover/[0-9a-fA-F-]+"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .cover()
            .cover_id(cover_id)
            .get()
            .send()
            .await?;

        assert_eq!(res.data.id, cover_id);
        assert_eq!(res.data.attributes.volume, Some("1".to_string()));
        assert_eq!(res.data.attributes.file_name, "1.jpg".to_string());
        assert_eq!(res.data.attributes.description, description);
        assert_eq!(res.data.attributes.locale, Some(Language::English));
        assert_eq!(res.data.attributes.version, 1);
        assert_eq!(
            res.data.attributes.created_at.to_string(),
            datetime.to_string()
        );
        assert_eq!(
            res.data.attributes.updated_at.as_ref().unwrap().to_string(),
            datetime.to_string()
        );

        Ok(())
    }

    #[tokio::test]
    async fn get_chapter_handles_404() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let cover_id = Uuid::new_v4();
        let error_id = Uuid::new_v4();

        let response_body = json!({
            "result": "error",
            "errors": [{
                "id": error_id.to_string(),
                "status": 404,
                "title": "Not found",
                "detail": "Cover could not be found"
            }]
        });

        Mock::given(method("GET"))
            .and(path_regex(r"/cover/[0-9a-fA-F-]+"))
            .respond_with(ResponseTemplate::new(404).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .cover()
            .cover_id(cover_id)
            .get()
            .send()
            .await
            .expect_err("expected error");

        if let Error::Api(errors) = res {
            assert_eq!(errors.errors.len(), 1);

            assert_eq!(errors.errors[0].id, error_id);
            assert_eq!(errors.errors[0].status, 404);
            assert_eq!(errors.errors[0].title, Some("Not found".to_string()));
            assert_eq!(
                errors.errors[0].detail,
                Some("Cover could not be found".to_string())
            );
        }

        Ok(())
    }
}
