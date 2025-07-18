//! Builder for the create manga endpoint.
//!
//! <https://api.mangadex.org/docs/swagger.html#/Manga/post-manga>
//!
//! ```rust
//! use std::collections::HashMap;
//!
//! use mangadex_api::MangaDexClient;
//! // use mangadex_api_types::{Password, Username};
//! use mangadex_api_types::Language;
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! /*
//!     Put your login script here   
//!     let _login_res = client
//!         .auth()
//!         .login()
//!         .username(Username::parse("myusername")?)
//!         .password(Password::parse("hunter23")?)
//!         .build()?
//!         .send()
//!         .await?;
//! */
//! let manga_res = client
//!     .manga()
//!     .post()
//!     .add_title((Language::English, "My New Manga Title".to_string()))
//!     .send()
//!     .await?;
//!
//! println!("Manga creation: {:?}", manga_res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;
use mangadex_api_schema::v5::{LocalizedString, MangaData};
use mangadex_api_types::{ContentRating, Demographic, Language, MangaLinks, MangaStatus};

/// Create a new manga.
///
/// This requires authentication.
///
/// Makes a request to `POST /manga`.
#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
#[derive(Debug, Serialize, Clone, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(
    setter(into, strip_option),
    build_fn(error = "crate::error::BuilderError")
)]
#[non_exhaustive]
pub struct CreateManga {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub http_client: HttpClientRef,

    #[builder(setter(each = "add_title"))]
    pub title: LocalizedString,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub alt_titles: Option<Vec<LocalizedString>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub description: Option<LocalizedString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub authors: Option<Vec<Uuid>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub artists: Option<Vec<Uuid>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub links: Option<MangaLinks>,
    pub original_language: Language,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub last_volume: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub last_chapter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub publication_demographic: Option<Option<Demographic>>,
    pub status: MangaStatus,
    /// Year the manga was released.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub year: Option<Option<u16>>,
    pub content_rating: ContentRating,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapter_numbers_reset_on_new_volume: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tags: Option<Vec<Uuid>>,
    /// Cover ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub primary_cover: Option<Option<Uuid>>,
    /// >= 1
    pub version: u32,
}

endpoint! {
    POST "/manga",
    #[body auth] CreateManga,
    #[rate_limited] MangaData,
    CreateMangaBuilder
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use time::OffsetDateTime;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{body_json, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::{
        ContentRating, Demographic, Language, MangaDexDateTime, MangaStatus, ResponseType, Tag,
    };

    #[tokio::test]
    async fn create_manga_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(non_exhaustive::non_exhaustive!(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            }))
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let manga_id = Uuid::new_v4();
        let tag_id: Uuid = Tag::Action.into();
        let manga_title = "Test Manga".to_string();

        let datetime = MangaDexDateTime::new(&OffsetDateTime::now_utc());

        let _expected_body = json!({
            "title": {
                "en": manga_title
            },
            "originalLanguage": "ja",
            "publicationDemographic": "shounen",
            "status": "ongoing",
            "contentRating": "safe",
            "tags": [tag_id],
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
                    "state": "draft",
                    "createdAt": datetime.to_string(),
                    "updatedAt": datetime.to_string(),

                    "version": 1
                },
                "relationships": []
            }
        });

        Mock::given(method("POST"))
            .and(path("/manga"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .and(header("Content-Type", "application/json"))
            .and(body_json(_expected_body))
            .respond_with(
                ResponseTemplate::new(201)
                    .insert_header("x-ratelimit-retry-after", "1698723860")
                    .insert_header("x-ratelimit-limit", "40")
                    .insert_header("x-ratelimit-remaining", "39")
                    .set_body_json(response_body),
            )
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .manga()
            .post()
            .add_title((Language::English, manga_title.clone()))
            .original_language(Language::Japanese)
            .publication_demographic(Demographic::Shounen)
            .status(MangaStatus::Ongoing)
            .content_rating(ContentRating::Safe)
            .tags(vec![Tag::Action.into()])
            .version(1_u32)
            .send()
            .await?;
        let res = res.body;
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
        assert!(res.data.attributes.chapter_numbers_reset_on_new_volume);
        assert_eq!(
            res.data.attributes.available_translated_languages[0],
            Language::English
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
    async fn create_manga_does_not_include_last_volume_when_not_used() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(non_exhaustive::non_exhaustive!(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            }))
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let manga_id = Uuid::new_v4();
        let tag_id: Uuid = Tag::Action.into();
        let manga_title = "Test Manga".to_string();

        let datetime = MangaDexDateTime::new(&OffsetDateTime::now_utc());

        let expected_body = json!({
            "title": {
                "en": manga_title
            },
            "originalLanguage": "ja",
            "publicationDemographic": "shounen",
            "status": "ongoing",
            "contentRating": "safe",
            "tags": [tag_id],
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
                    "state": "draft",
                    "createdAt": datetime.to_string(),
                    "updatedAt": datetime.to_string(),

                    "version": 1
                },
                "relationships": []
            }
        });

        Mock::given(method("POST"))
            .and(path("/manga"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .and(header("Content-Type", "application/json"))
            .and(body_json(expected_body))
            .respond_with(
                ResponseTemplate::new(201)
                    .insert_header("x-ratelimit-retry-after", "1698723860")
                    .insert_header("x-ratelimit-limit", "40")
                    .insert_header("x-ratelimit-remaining", "39")
                    .set_body_json(response_body),
            )
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .manga()
            .post()
            .add_title((Language::English, manga_title.clone()))
            .original_language(Language::Japanese)
            .status(MangaStatus::Ongoing)
            .content_rating(ContentRating::Safe)
            .tags(vec![Tag::Action.into()])
            .publication_demographic(Demographic::Shounen)
            .version(1_u32)
            .send()
            .await?;

        let res = res.body;

        assert_eq!(res.data.attributes.last_volume, None);

        Ok(())
    }

    #[tokio::test]
    async fn create_manga_sends_null_last_volume() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(non_exhaustive::non_exhaustive!(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            }))
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let manga_id = Uuid::new_v4();
        let tag_id: Uuid = Tag::Action.into();
        let manga_title = "Test Manga".to_string();

        let datetime = MangaDexDateTime::new(&OffsetDateTime::now_utc());

        let expected_body = json!({
            "title": {
                "en": manga_title
            },
            "originalLanguage": "ja",
            "publicationDemographic": "shounen",
            "status": "ongoing",
            "contentRating": "safe",
            "tags": [tag_id],
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
                    "state": "draft",
                    "createdAt": datetime.to_string(),
                    "updatedAt": datetime.to_string(),

                    "version": 1
                },
                "relationships": []
            }
        });

        Mock::given(method("POST"))
            .and(path("/manga"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .and(header("Content-Type", "application/json"))
            .and(body_json(expected_body))
            .respond_with(
                ResponseTemplate::new(201)
                    .insert_header("x-ratelimit-retry-after", "1698723860")
                    .insert_header("x-ratelimit-limit", "40")
                    .insert_header("x-ratelimit-remaining", "39")
                    .set_body_json(response_body),
            )
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .manga()
            .post()
            .add_title((Language::English, manga_title.clone()))
            .original_language(Language::Japanese)
            .status(MangaStatus::Ongoing)
            .content_rating(ContentRating::Safe)
            .publication_demographic(Demographic::Shounen)
            .tags(vec![Tag::Action.into()])
            .version(1_u32)
            .send()
            .await?;

        let res = res.body;

        assert_eq!(res.data.attributes.last_volume, None);

        Ok(())
    }

    #[tokio::test]
    async fn create_manga_sends_last_volume_with_value() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(non_exhaustive::non_exhaustive!(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            }))
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let manga_id = Uuid::new_v4();
        let tag_id: Uuid = Tag::Action.into();
        let manga_title = "Test Manga".to_string();

        let datetime = MangaDexDateTime::new(&OffsetDateTime::now_utc());

        let expected_body = json!({
            "title": {
                "en": manga_title
            },
            "originalLanguage": "ja",
            "lastVolume": "1",
            "publicationDemographic": "shounen",
            "status": "ongoing",
            "contentRating": "safe",
            "tags": [tag_id],
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
                    "lastVolume": "1",
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
                    "state": "draft",
                    "createdAt": datetime.to_string(),
                    "updatedAt": datetime.to_string(),

                    "version": 1
                },
                "relationships": []
            }
        });

        Mock::given(method("POST"))
            .and(path("/manga"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .and(header("Content-Type", "application/json"))
            .and(body_json(expected_body))
            .respond_with(
                ResponseTemplate::new(201)
                    .insert_header("x-ratelimit-retry-after", "1698723860")
                    .insert_header("x-ratelimit-limit", "40")
                    .insert_header("x-ratelimit-remaining", "39")
                    .set_body_json(response_body),
            )
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .manga()
            .post()
            .add_title((Language::English, manga_title.clone()))
            .original_language(Language::Japanese)
            .last_volume("1")
            .publication_demographic(Demographic::Shounen)
            .status(MangaStatus::Ongoing)
            .content_rating(ContentRating::Safe)
            .tags(vec![Tag::Action.into()])
            .version(1_u32)
            .send()
            .await?;

        let res = res.body;

        assert_eq!(res.data.attributes.last_volume, Some("1".to_string()));

        Ok(())
    }
}
