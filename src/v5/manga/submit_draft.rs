//! Builder for submitting (committing) a Manga Draft.
//!
//! This endpoint requires authentication.
//!
//! A Manga Draft that is to be submitted must have at least one cover, must be in the "draft" state and must be passed the correct version in the request body.
//!
//! <https://api.mangadex.org/swagger.html#/Manga/commit-manga-draft>
//!
//! # Examples
//!
//! ```rust
//! use uuid::Uuid;
//!
//! use mangadex_api::v5::MangaDexClient;
//! use mangadex_api_types::{Password, Username};
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! let _login_res = client
//!     .auth()
//!     .login()
//!     .username(Username::parse("myusername")?)
//!     .password(Password::parse("hunter23")?)
//!     .build()?
//!     .send()
//!     .await?;
//!
//! let manga_id = Uuid::new_v4();
//! let res = client
//!     .manga()
//!     .submit_draft()
//!     .manga_id(&manga_id)
//!     .version(1_u32)
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("submitted manga draft: {:?}", res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;
use mangadex_api_schema::v5::MangaResponse;

#[derive(Debug, Builder, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into))]
pub struct SubmitMangaDraft<'a> {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    pub(crate) http_client: HttpClientRef,

    #[serde(skip)]
    pub manga_id: &'a Uuid,

    pub version: u32,
}

endpoint! {
    POST ("/manga/draft/{}/commit/", manga_id),
    #[body auth] SubmitMangaDraft<'_>,
    #[flatten_result] MangaResponse
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use time::OffsetDateTime;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{body_json, header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::error::Error;
    use mangadex_api_types::{
        ContentRating, Demographic, Language, MangaDexDateTime, MangaStatus, ResponseType, Tag,
    };

    #[tokio::test]
    async fn submit_manga_draft_fires_a_request_to_base_url() -> anyhow::Result<()> {
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
        let tag_id: Uuid = Tag::Action.into();
        let manga_title = "Test Manga".to_string();

        let datetime = MangaDexDateTime::new(&OffsetDateTime::now_utc());

        let expected_body = json!({
            "version": 1
        });
        let response_body = json!({
            "result": "ok",
            "response": "entity",
            "data": {
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
                    "publicationDemographic": "shounen",
                    "status": "ongoing",
                    "year": null,
                    "contentRating": "safe",
                    "chapterNumbersResetOnNewVolume": true,
                    "availableTranslatedLanguages": ["en"],
                    "tags": [
                        {
                            "id": tag_id,
                            "type": "tag",
                            "attributes": {
                                "name": {
                                    "en": "Action"
                                },
                                "description": [],
                                "group": "genre",
                                "version": 1
                            },
                            "relationships": []
                        }
                    ],
                    "state": "submitted",
                    "createdAt": datetime.to_string(),
                    "updatedAt": datetime.to_string(),

                    "version": 1
                },
                "relationships": []
            }
        });

        Mock::given(method("POST"))
            .and(path_regex(r"/manga/draft/[0-9a-fA-F-]+/commit"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .and(header("Content-Type", "application/json"))
            .and(body_json(expected_body))
            .respond_with(ResponseTemplate::new(201).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .manga()
            .submit_draft()
            .manga_id(&manga_id)
            .version(1_u32)
            .build()?
            .send()
            .await?;

        assert_eq!(res.response, ResponseType::Entity);
        assert_eq!(res.data.id, manga_id);
        assert_eq!(
            res.data.attributes.title.get(&Language::English).unwrap(),
            &manga_title
        );
        assert!(res.data.attributes.alt_titles.is_empty());
        assert!(res.data.attributes.description.is_empty());
        assert!(!res.data.attributes.is_locked);
        assert_eq!(res.data.attributes.links, None);
        assert_eq!(res.data.attributes.original_language, Language::Japanese);
        assert_eq!(res.data.attributes.last_volume, None);
        assert_eq!(res.data.attributes.last_chapter, None);
        assert_eq!(
            res.data.attributes.publication_demographic.unwrap(),
            Demographic::Shounen
        );
        assert_eq!(res.data.attributes.status, MangaStatus::Ongoing);
        assert_eq!(res.data.attributes.year, None);
        assert_eq!(
            res.data.attributes.content_rating.unwrap(),
            ContentRating::Safe
        );
        assert_eq!(
            res.data.attributes.tags[0]
                .attributes
                .name
                .get(&Language::English),
            Some(&"Action".to_string())
        );
        assert_eq!(
            res.data.attributes.created_at.to_string(),
            datetime.to_string()
        );
        assert_eq!(
            res.data.attributes.updated_at.as_ref().unwrap().to_string(),
            datetime.to_string()
        );
        assert_eq!(res.data.attributes.version, 1);

        Ok(())
    }

    #[tokio::test]
    async fn submit_manga_draft_requires_auth() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let manga_id = Uuid::new_v4();
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

        Mock::given(method("POST"))
            .and(path_regex(r"/manga/draft/[0-9a-fA-F-]+/commit"))
            .and(header("Content-Type", "application/json"))
            .respond_with(ResponseTemplate::new(403).set_body_json(response_body))
            .expect(0)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .manga()
            .submit_draft()
            .manga_id(&manga_id)
            .version(1_u32)
            .build()?
            .send()
            .await
            .expect_err("expected error");

        match res {
            Error::MissingTokens => {}
            _ => panic!("unexpected error: {:#?}", res),
        }

        Ok(())
    }
}
