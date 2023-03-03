//! Builder for the chapter update endpoint.
//!
//! <https://api.mangadex.org/swagger.html#/Manga/put-chapter-id>
//!
//! # Examples
//!
//! ```rust
//! use std::collections::HashMap;
//!
//! use uuid::Uuid;
//!
//! use mangadex_api::types::Language;
//! use mangadex_api::v5::MangaDexClient;
//! use mangadex_api::types::{Password, Username};
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
//! let mut manga_titles = HashMap::new();
//! manga_titles.insert(Language::English, "Updated Manga Title".to_string());
//! let res = client
//!     .manga()
//!     .update()
//!     .manga_id(&manga_id)
//!     .title(manga_titles)
//!     .version(2u32)
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("update: {:?}", res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;
use mangadex_api_schema::v5::{LocalizedString, MangaResponse};
use mangadex_api_types::{ContentRating, Demographic, Language, MangaLinks, MangaStatus, Tag};

/// Update a manga's information.
///
/// All fields that are not changing should still have the field populated with the old information
/// so that it is not set as `null` on the server.
#[derive(Debug, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option))]
#[non_exhaustive]
pub struct UpdateManga<'a> {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    pub(crate) http_client: HttpClientRef,

    #[serde(skip)]
    pub manga_id: &'a Uuid,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title: Option<LocalizedString>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub original_language: Option<Language>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub last_volume: Option<Option<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub last_chapter: Option<Option<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub publication_demographic: Option<Option<Demographic>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub status: Option<MangaStatus>,
    /// Year the manga was released.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub year: Option<Option<u16>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub content_rating: Option<ContentRating>,
    #[builder(default)]
    pub chapter_numbers_reset_on_new_volume: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tags: Option<Vec<Tag>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub primary_cover: Option<Option<&'a Uuid>>,
    /// >= 1
    pub version: u32,
}

endpoint! {
    PUT ("/manga/{}", manga_id),
    #[body auth] UpdateManga<'_>,
    #[flatten_result] MangaResponse
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::json;
    use time::OffsetDateTime;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::{ContentRating, Language, MangaDexDateTime, MangaStatus, Tag};

    #[tokio::test]
    async fn update_chapter_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let manga_id = Uuid::new_v4();
        let mut title = HashMap::new();
        title.insert(Language::English, "New Manga Title".to_string());
        let datetime = MangaDexDateTime::new(&OffsetDateTime::now_utc());

        let _expected_body = json!({
            "title": {
                "en": "New Manga Title"
            },
            "altTitles": null,
            "description": null,
            "authors": null,
            "artists": null,
            "links": null,
            "originalLanguage": null,
            "lastVolume": null,
            "lastChapter": null,
            "publicationDemographic": null,
            "status": null,
            "year": null,
            "contentRating": null,
            "chapterNumbersResetOnNewVolume": false,
            "availableTranslatedLanguages": ["en"],
            "tags": null,
            "modNotes": null,
            "version": 2
        });
        let response_body = json!({
            "result": "ok",
            "response": "entity",
            "data": {
                "id": manga_id,
                "type": "chapter",
                "attributes": {
                    "title": {
                        "en": "New Manga Title"
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
                    "state": "published",
                    "version": 2,
                    "createdAt": datetime.to_string(),
                    "updatedAt": datetime.to_string(),
                },
                "relationships": []
            }
        });

        Mock::given(method("PUT"))
            .and(path_regex(r"/manga/[0-9a-fA-F-]+"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .and(header("Content-Type", "application/json"))
            // TODO: Make the request body check work.
            // .and(body_json(expected_body))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _ = mangadex_client
            .manga()
            .update()
            .manga_id(&manga_id)
            .title(title)
            .version(2_u32)
            .build()?
            .send()
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn update_manga_does_not_include_last_volume_when_not_used() -> anyhow::Result<()> {
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

        let _expected_body = json!({
            "originalLanguage": "ja",
            "lastVolume": null,
            "publicationDemographic": "shounen",
            "status": "ongoing",
            "contentRating": "safe",
            "tags": [tag_id]
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

        Mock::given(method("PUT"))
            .and(path_regex(r"/manga/[0-9a-fA-F-]+"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .and(header("Content-Type", "application/json"))
            // TODO: Make the request body check work.
            // .and(body_json(expected_body))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .manga()
            .update()
            .manga_id(&manga_id)
            .original_language(Language::Japanese)
            .status(MangaStatus::Ongoing)
            .content_rating(ContentRating::Safe)
            .tags(vec![Tag::Action])
            .version(1_u32)
            .build()?
            .send()
            .await?;

        assert_eq!(res.data.attributes.last_volume, None);

        Ok(())
    }

    #[tokio::test]
    async fn update_manga_sends_null_last_volume() -> anyhow::Result<()> {
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

        let _expected_body = json!({
            "originalLanguage": "ja",
            "lastVolume": null,
            "publicationDemographic": "shounen",
            "status": "ongoing",
            "contentRating": "safe",
            "chapterNumbersResetOnNewVolume": false,
            "tags": [tag_id]
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

        Mock::given(method("PUT"))
            .and(path_regex(r"/manga/[0-9a-fA-F-]+"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .and(header("Content-Type", "application/json"))
            // TODO: Make the request body check work.
            // .and(body_json(expected_body))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .manga()
            .update()
            .manga_id(&manga_id)
            .original_language(Language::Japanese)
            .last_volume(None)
            .status(MangaStatus::Ongoing)
            .content_rating(ContentRating::Safe)
            .tags(vec![Tag::Action])
            .version(1_u32)
            .build()?
            .send()
            .await?;

        assert_eq!(res.data.attributes.last_volume, None);

        Ok(())
    }

    #[tokio::test]
    async fn update_manga_sends_last_volume_with_value() -> anyhow::Result<()> {
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

        let _expected_body = json!({
            "originalLanguage": "ja",
            "lastVolume": "1",
            "publicationDemographic": "shounen",
            "status": "ongoing",
            "contentRating": "safe",
            "chapterNumbersResetOneNewVolume": false,
            "tags": [tag_id]
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

        Mock::given(method("PUT"))
            .and(path_regex(r"/manga/[0-9a-fA-F-]+"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .and(header("Content-Type", "application/json"))
            // TODO: Make the request body check work.
            // .and(body_json(expected_body))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .manga()
            .update()
            .manga_id(&manga_id)
            .original_language(Language::Japanese)
            .last_volume("1")
            .status(MangaStatus::Ongoing)
            .content_rating(ContentRating::Safe)
            .tags(vec![Tag::Action])
            .version(1_u32)
            .build()?
            .send()
            .await?;

        assert_eq!(res.data.attributes.last_volume, Some("1".to_string()));

        Ok(())
    }
}
