//! Builder for the random manga endpoint.
//!
//! <https://api.mangadex.org/docs/swagger.html#/Manga/get-manga-random>
//!
//! # Examples
//!
//! ```rust
//! use mangadex_api::v5::MangaDexClient;
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! let manga_res = client
//!     .manga()
//!     .random()
//!     .get()
//!     .send()
//!     .await?;
//!
//! println!("random manga: {:?}", manga_res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use mangadex_api_schema::v5::MangaData;
use mangadex_api_types::{ContentRating, ReferenceExpansionResource, TagSearchMode};
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;

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
pub struct GetRandomManga {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub(crate) http_client: HttpClientRef,

    #[builder(setter(each = "include"), default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub includes: Vec<ReferenceExpansionResource>,

    /// Ensure the returned Manga is one of the given content ratings.
    ///
    /// If this is not set, the default ratings MangaDex will use are:
    ///     - safe
    ///     - suggestive
    ///     - erotica
    #[builder(setter(each = "add_content_rating"), default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub content_rating: Vec<ContentRating>,

    #[builder(setter(each = "include_tag"), default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub included_tags: Vec<Uuid>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included_tags_mode: Option<TagSearchMode>,

    #[builder(setter(each = "exclude_tag"), default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub excluded_tags: Vec<Uuid>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_tags_mode: Option<TagSearchMode>,
}

endpoint! {
    GET ("/manga/random"),
    #[query] GetRandomManga,
    #[rate_limited] MangaData,
    GetRandomMangaBuilder
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use time::OffsetDateTime;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::error::Error;
    use mangadex_api_types::MangaDexDateTime;

    #[tokio::test]
    async fn get_random_manga_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
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
                    "links": null,
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
                    "version": 1,
                    "createdAt": datetime.to_string(),
                    "updatedAt": datetime.to_string(),
                },
                "relationships": []
            }
        });

        Mock::given(method("GET"))
            .and(path(r"/manga/random"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("x-ratelimit-retry-after", "1698723860")
                    .insert_header("x-ratelimit-limit", "40")
                    .insert_header("x-ratelimit-remaining", "39")
                    .set_body_json(response_body),
            )
            .expect(1)
            .mount(&mock_server)
            .await;

        let _ = mangadex_client.manga().random().get().send().await?;

        Ok(())
    }

    #[tokio::test]
    async fn get_random_manga_deserialize_handles_empty_array_links_field() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
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
                    "links": [],
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
                    "version": 1,
                    "createdAt": datetime.to_string(),
                    "updatedAt": datetime.to_string(),
                },
                "relationships": []
            }
        });

        Mock::given(method("GET"))
            .and(path(r"/manga/random"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("x-ratelimit-retry-after", "1698723860")
                    .insert_header("x-ratelimit-limit", "40")
                    .insert_header("x-ratelimit-remaining", "39")
                    .set_body_json(response_body),
            )
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client.manga().random().get().send().await?;

        assert!(res.data.attributes.links.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn get_random_manga_deserialize_handles_links_field() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
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
                    "links": {
                        "bw": "1",
                        "ebj": "https://ebookjapan.yahoo.co.jp/",
                        "cdj": "https://www.cdjapan.co.jp/",
                        "raw": "https://miku.mangadex.org",
                        "engtl": "https://mangadex.org",
                        "kt": "1",
                        "al": "1",
                        "ap": "a",
                        "nu": "a",
                        "mal": "1"
                    },
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
                    "version": 1,
                    "createdAt": datetime.to_string(),
                    "updatedAt": datetime.to_string(),
                },
                "relationships": []
            }
        });

        Mock::given(method("GET"))
            .and(path(r"/manga/random"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("x-ratelimit-retry-after", "1698723860")
                    .insert_header("x-ratelimit-limit", "40")
                    .insert_header("x-ratelimit-remaining", "39")
                    .set_body_json(response_body),
            )
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client.manga().random().get().send().await?;

        if let Some(links) = &res.data.attributes.links {
            assert_eq!(links.book_walker.clone().unwrap().0, "1".to_string());
            assert_eq!(links.manga_updates, None);
        } else {
            panic!("error deserializing 'links' field");
        }

        Ok(())
    }

    #[tokio::test]
    async fn get_random_manga_handles_http_503() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        Mock::given(method("GET"))
            .and(path(r"/manga/random"))
            .respond_with(
                ResponseTemplate::new(503)
                    .insert_header("x-ratelimit-retry-after", "1698723860")
                    .insert_header("x-ratelimit-limit", "40")
                    .insert_header("x-ratelimit-remaining", "39"),
            )
            .expect(1)
            .mount(&mock_server)
            .await;

        match mangadex_client.manga().random().get().send().await {
            Err(Error::ServerError(..)) => {}
            _ => panic!("expected server error"),
        }

        Ok(())
    }
}
