//! Builder for getting a specific Manga Draft.
//!
//! This endpoint requires authentication.
//!
//! This endpoint is largely identical to the Manga Get endpoint except that this fetches Manga
//! that is not in the "published" state.
//!
//! <https://api.mangadex.org/swagger.html#/Manga/get-manga-id-draft>
//!
//! # Examples
//!
//! ```rust
//! use uuid::Uuid;
//!
//! use mangadex_api::MangaDexClient;
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
//! let manga_res = client
//!     .manga()
//!     .get_draft()
//!     .manga_id(&manga_id)
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("manga draft view: {:?}", manga_res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;
use mangadex_api_schema::v5::MangaResponse;
use mangadex_api_types::ReferenceExpansionResource;

#[derive(Debug, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), pattern = "owned")]
pub struct GetMangaDraft<'a> {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    pub(crate) http_client: HttpClientRef,

    #[serde(skip)]
    pub manga_id: &'a Uuid,

    #[builder(setter(each = "include"), default)]
    pub includes: Vec<ReferenceExpansionResource>,
}

endpoint! {
    GET ("/manga/draft/{}", manga_id),
    #[query auth] GetMangaDraft<'_>,
    #[flatten_result] MangaResponse
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use time::OffsetDateTime;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_schema::v5::RelatedAttributes;
    use mangadex_api_types::error::Error;
    use mangadex_api_types::{
        MangaDexDateTime, MangaRelation, ReferenceExpansionResource, RelationshipType,
    };

    #[tokio::test]
    async fn get_manga_draft_fires_a_request_to_base_url() -> anyhow::Result<()> {
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

        let datetime = MangaDexDateTime::new(&OffsetDateTime::now_utc());

        let response_body = json!({
            "result": "ok",
            "response": "entity",
            "data": {
                "id": manga_id,
                "type": "manga",
                "attributes": {
                    "title": {
                        "en": "Test Manga"
                    },
                    "altTitles": [],
                    "description": {},
                    "isLocked": false,
                    "links": {},
                    "originalLanguage": "ja",
                    "lastVolume": "1",
                    "lastChapter": "1",
                    "publicationDemographic": "shoujo",
                    "status": "completed",
                    "year": 2021,
                    "contentRating": "safe",
                    "chapterNumbersResetOnNewVolume": true,
                    "availableTranslatedLanguages": ["en"],
                    "tags": [],
                    "state": "rejected",
                    "version": 1,
                    "createdAt": datetime.to_string(),
                    "updatedAt": datetime.to_string(),
                },
                "relationships": [
                    {
                        "id": "a3219a4f-73c0-4213-8730-05985130539a",
                        "type": "manga",
                        "related": "side_story",
                    }
                ]
            }
        });

        Mock::given(method("GET"))
            .and(path_regex(r"/manga/draft/[0-9a-fA-F-]+"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .manga()
            .get_draft()
            .manga_id(&manga_id)
            .build()?
            .send()
            .await?;

        assert_eq!(res.data.relationships[0].type_, RelationshipType::Manga);
        assert_eq!(
            res.data.relationships[0].related,
            Some(MangaRelation::SideStory)
        );
        assert!(res.data.relationships[0].attributes.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn get_manga_draft_handles_reference_expansion() -> anyhow::Result<()> {
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

        let datetime = MangaDexDateTime::new(&OffsetDateTime::now_utc());

        let response_body = json!({
            "result": "ok",
            "response": "entity",
            "data": {
                "id": manga_id,
                "type": "manga",
                "attributes": {
                    "title": {
                        "en": "Test Manga"
                    },
                    "altTitles": [],
                    "description": {},
                    "isLocked": false,
                    "links": {},
                    "originalLanguage": "ja",
                    "lastVolume": "1",
                    "lastChapter": "1",
                    "publicationDemographic": "shoujo",
                    "status": "completed",
                    "year": 2021,
                    "contentRating": "safe",
                    "chapterNumbersResetOnNewVolume": true,
                    "availableTranslatedLanguages": ["en"],
                    "tags": [],
                    "state": "submitted",
                    "version": 1,
                    "createdAt": datetime.to_string(),
                    "updatedAt": datetime.to_string(),
                },
                "relationships": [
                    {
                        "id": "fc343004-569b-4750-aba0-05ab35efc17c",
                        "type": "author",
                        "attributes": {
                            "name": "Hologfx",
                            "imageUrl": null,
                            "biography": [],
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
                            "website": null,
                            "createdAt": "2021-04-19T21:59:45+00:00",
                            "updatedAt": "2021-04-19T21:59:45+00:00",
                            "version": 1
                        }
                    }
                ]
            }
        });

        Mock::given(method("GET"))
            .and(path_regex(r"/manga/draft/[0-9a-fA-F-]+"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .manga()
            .get_draft()
            .manga_id(&manga_id)
            .include(&ReferenceExpansionResource::Author)
            .build()?
            .send()
            .await?;

        assert_eq!(res.data.relationships[0].type_, RelationshipType::Author);
        assert!(res.data.relationships[0].related.is_none());
        match res.data.relationships[0].attributes.as_ref().unwrap() {
            RelatedAttributes::Author(author) => assert_eq!(author.name, "Hologfx".to_string()),
            _ => panic!("Expected author RelatedAttributes"),
        }

        Ok(())
    }

    #[tokio::test]
    async fn get_manga_draft_requires_auth() -> anyhow::Result<()> {
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

        Mock::given(method("GET"))
            .and(path_regex(r"/manga/draft/[0-9a-fA-F-]+"))
            .respond_with(ResponseTemplate::new(403).set_body_json(response_body))
            .expect(0)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .manga()
            .get_draft()
            .manga_id(&manga_id)
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
