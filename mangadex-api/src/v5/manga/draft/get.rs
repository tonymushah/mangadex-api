//! Builder for getting a list of Manga Drafts.
//!
//! This endpoint requires authentication.
//!
//! <https://api.mangadex.org/docs/swagger.html#/Manga/get-manga-drafts>
//!
//! # Examples
//!
//! ```rust
//! use mangadex_api_types::{MangaState, MangaStatus};
//! use mangadex_api::v5::MangaDexClient;
//! // use mangadex_api_types::{Password, Username};
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! /*
//!     let _login_res = client
//!         .auth()
//!         .login()
//!         .post()    
//!         .username(Username::parse("myusername")?)
//!         .password(Password::parse("hunter23")?)
//!         .build()?
//!         .send()
//!         .await?;
//! */
//! let manga_res = client
//!     .manga()
//!     .draft()
//!     .get()
//!     .state(MangaState::Draft)
//!     .send()
//!     .await?;
//!
//! println!("manga: {:?}", manga_res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use mangadex_api_schema::v5::MangaCollection;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;
use mangadex_api_types::{MangaDraftsSortOrder, MangaState, ReferenceExpansionResource};

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
pub struct ListMangaDrafts {
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub http_client: HttpClientRef,

    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Default: 10 (if not specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// >= 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[deprecated(since = "1.2.1", note = "MangaDex removed this in 5.4.9 of their API")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<MangaState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<MangaDraftsSortOrder>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(setter(each = "include"))]
    pub includes: Vec<ReferenceExpansionResource>,
}

endpoint! {
    GET "/manga/draft",
    #[query auth] ListMangaDrafts,
    #[flatten_result] crate::Result<MangaCollection>,
    ListMangaDraftsBuilder
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use time::OffsetDateTime;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::error::Error;
    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::{
        ContentRating, Demographic, Language, MangaDexDateTime, MangaStatus, ResponseType,
    };

    #[tokio::test]
    async fn list_manga_drafts_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let manga_id = Uuid::new_v4();
        let manga_title = "Test Manga".to_string();

        let datetime = MangaDexDateTime::new(&OffsetDateTime::now_utc());

        let response_body = json!({
            "result": "ok",
            "response": "collection",
            "data": [
                {
                    "id": manga_id,
                    "type": "manga",
                    "attributes": {
                        "title": {
                            "en": manga_title
                        },
                        "altTitles": [],
                        "description": {},
                        "isLocked": false,
                        "links": null,
                        "originalLanguage": "ja",
                        "lastVolume": null,
                        "lastChapter": null,
                        "publicationDemographic": "shoujo",
                        "status": "ongoing",
                        "year": null,
                        "contentRating": "safe",
                        "chapterNumbersResetOnNewVolume": true,
                        "availableTranslatedLanguages": ["en"],
                        "tags": [],
                        "state": "draft",
                        "createdAt": datetime.to_string(),
                        "updatedAt": datetime.to_string(),

                        "version": 1
                    },
                    "relationships": []
                }
            ],
            "limit": 1,
            "offset": 0,
            "total": 1
        });

        Mock::given(method("GET"))
            .and(path("/manga/draft"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .manga()
            .draft()
            .get()
            .limit(1u32)
            .send()
            .await?;

        assert_eq!(res.response, ResponseType::Collection);
        let manga = &res.data[0];
        assert_eq!(manga.id, manga_id);
        assert_eq!(
            manga.attributes.title.get(&Language::English).unwrap(),
            &manga_title
        );
        assert!(manga.attributes.alt_titles.is_empty());
        assert!(manga.attributes.description.is_empty());
        assert!(!manga.attributes.is_locked);
        assert_eq!(manga.attributes.links, None);
        assert_eq!(manga.attributes.original_language, Language::Japanese);
        assert_eq!(manga.attributes.last_volume, None);
        assert_eq!(manga.attributes.last_chapter, None);
        assert_eq!(
            manga.attributes.publication_demographic.unwrap(),
            Demographic::Shoujo
        );
        assert_eq!(manga.attributes.status, MangaStatus::Ongoing);
        assert_eq!(manga.attributes.year, None);
        assert_eq!(
            manga.attributes.content_rating.unwrap(),
            ContentRating::Safe
        );
        assert!(manga.attributes.tags.is_empty());
        assert_eq!(
            manga.attributes.created_at.to_string(),
            datetime.to_string()
        );
        assert_eq!(
            manga.attributes.updated_at.as_ref().unwrap().to_string(),
            datetime.to_string()
        );
        assert_eq!(manga.attributes.version, 1);

        Ok(())
    }

    #[tokio::test]
    async fn list_manga_drafts_handles_400() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
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
            .and(path("/manga/draft"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .respond_with(ResponseTemplate::new(400).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .manga()
            .draft()
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

    #[tokio::test]
    async fn list_manga_drafts_requires_auth() -> anyhow::Result<()> {
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
                "status": 403,
                "title": "Forbidden",
                "detail": "You must be logged in to continue."
            }]
        });

        Mock::given(method("GET"))
            .and(path("/manga/draft"))
            .respond_with(ResponseTemplate::new(403).set_body_json(response_body))
            .expect(0)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .manga()
            .draft()
            .get()
            .limit(0u32)
            .send()
            .await
            .expect_err("expected error");

        match res {
            Error::MissingTokens => {}
            _ => panic!("unexpected error: {res:#?}"),
        }

        Ok(())
    }
}
