//! Builder for the author list endpoint.
//!
//! <https://api.mangadex.org/swagger.html#/Author/get-author>
//!
//! # Examples
//!
//! ```rust
//! use mangadex_api::v5::MangaDexClient;
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! let author_res = client
//!     .author()
//!     .list()
//!     .name("carlo zen")
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("authors: {:?}", author_res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;
use mangadex_api_schema::v5::AuthorListResponse;
use mangadex_api_types::{AuthorSortOrder, ReferenceExpansionResource};

#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
#[derive(Debug, Serialize, Clone, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default, pattern = "owned")]
#[non_exhaustive]
pub struct ListAuthor {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub(crate) http_client: HttpClientRef,

    pub limit: Option<u32>,
    pub offset: Option<u32>,
    #[serde(rename = "ids")]
    #[builder(setter(each = "add_author"))]
    pub author_ids: Vec<Uuid>,
    pub name: Option<String>,
    pub order: Option<AuthorSortOrder>,
    #[builder(setter(each = "include"))]
    pub includes: Vec<ReferenceExpansionResource>,
}

endpoint! {
    GET "/author",
    #[query] ListAuthor,
    #[flatten_result] AuthorListResponse
}

#[cfg(test)]
mod tests {
    use fake::faker::lorem::en::Sentence;
    use fake::faker::name::en::Name;
    use fake::Fake;
    use serde_json::json;
    use time::OffsetDateTime;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::error::Error;
    use mangadex_api_types::{Language, MangaDexDateTime, ResponseType};

    #[tokio::test]
    async fn list_author_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let author_id = Uuid::new_v4();
        let author_name: String = Name().fake();
        let author_biography: String = Sentence(1..2).fake();

        let datetime = MangaDexDateTime::new(&OffsetDateTime::now_utc());

        let response_body = json!({
            "result": "ok",
            "response": "collection",
            "data": [
                {
                    "id": author_id,
                    "type": "author",
                    "attributes": {
                        "name": author_name,
                        "imageUrl": "",
                        "biography": {
                            "en": author_biography,
                        },
                        "twitter": null,
                        "pixiv": null,
                        "melonBook": null,
                        "fanBox": null,
                        "booth": null,
                        "nicoVideo": null,
                        "skeb": null,
                        "fantia": null,
                        "tumblr": null,
                        "youtube": null,
                        "weibo": null,
                        "naver": null,
                        "website": null,
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
            .and(path("/author"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .author()
            .search()
            .limit(1u32)
            .build()?
            .send()
            .await?;

        assert_eq!(res.response, ResponseType::Collection);
        let author = &res.data[0];
        assert_eq!(author.id, author_id);
        assert_eq!(author.attributes.name, author_name);
        assert_eq!(author.attributes.image_url, Some("".to_string()));
        assert_eq!(
            author.attributes.biography.get(&Language::English),
            Some(&author_biography)
        );
        assert_eq!(author.attributes.version, 1);
        assert_eq!(
            author.attributes.created_at.to_string(),
            datetime.to_string()
        );
        assert_eq!(
            author.attributes.updated_at.as_ref().unwrap().to_string(),
            datetime.to_string()
        );

        Ok(())
    }

    #[tokio::test]
    async fn list_author_handles_400() -> anyhow::Result<()> {
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
            .and(path(r"/author"))
            .respond_with(ResponseTemplate::new(400).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .author()
            .search()
            .limit(0u32)
            .build()?
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
