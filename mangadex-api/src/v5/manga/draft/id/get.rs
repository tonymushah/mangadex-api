//! Builder for getting a specific Manga Draft.
//!
//! This endpoint requires authentication.
//!
//! This endpoint is largely identical to the Manga Get endpoint except that this fetches Manga
//! that is not in the "published" state.
//!
//! <https://api.mangadex.org/docs/swagger.html#/Manga/get-manga-id-draft>
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
//! /*
//!     // Put your login script here
//!     let _login_res = client
//!         .auth()
//!         .login()
//!         .post()
//!         .username(Username::parse("myusername")?)
//!         .password(Password::parse("hunter23")?)
//!         .send()
//!         .await?;
//!  */
//!
//! let manga_id = Uuid::new_v4();
//! let manga_res = client
//!     .manga()
//!     .draft()
//!     .id(manga_id)
//!     .get()
//!     .send()
//!     .await?;
//!
//! println!("manga draft view: {:?}", manga_res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use mangadex_api_schema::v5::MangaData;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;
use mangadex_api_types::ReferenceExpansionResource;

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
pub struct GetMangaDraft {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub http_client: HttpClientRef,

    #[serde(skip_serializing)]
    pub manga_id: Uuid,

    #[builder(setter(each = "include"), default)]
    pub includes: Vec<ReferenceExpansionResource>,
}

endpoint! {
    GET ("/manga/draft/{}", manga_id),
    #[query auth] GetMangaDraft,
    #[flatten_result] crate::Result<MangaData>,
    GetMangaDraftBuilder
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use time::OffsetDateTime;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::error::Error;
    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_schema::v5::RelatedAttributes;
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
            .draft()
            .id(manga_id)
            .get()
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
            .draft()
            .id(manga_id)
            .get()
            .include(ReferenceExpansionResource::Author)
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
            .draft()
            .id(manga_id)
            .get()
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
