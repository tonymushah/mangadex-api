//! Builder for the manga list endpoint.
//!
//! <https://api.mangadex.org/swagger.html#/Manga/get-search-manga>
//!
//! # Examples
//!
//! ```rust
//! use mangadex_api_types::MangaStatus;
//! use mangadex_api::v5::MangaDexClient;
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! let manga_res = client
//!     .manga()
//!     .list()
//!     .title("full metal")
//!     .add_status(MangaStatus::Completed)
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("manga: {:?}", manga_res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;
use mangadex_api_schema::v5::MangaListResponse;
use mangadex_api_types::{
    ContentRating, Demographic, Language, MangaDexDateTime, MangaSortOrder, MangaStatus,
    ReferenceExpansionResource, Tag, TagSearchMode,
};

#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
#[derive(Debug, Serialize, Clone, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default, pattern = "owned")]
#[non_exhaustive]
pub struct ListManga {
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub(crate) http_client: HttpClientRef,

    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub title: Option<String>,
    #[builder(setter(each = "add_author"))]
    pub authors: Vec<Uuid>,
    #[builder(setter(each = "add_artist"))]
    pub artists: Vec<Uuid>,
    pub year: Option<u16>,
    #[builder(setter(each = "include_tag"))]
    pub included_tags: Vec<Tag>,
    pub included_tags_mode: Option<TagSearchMode>,
    #[builder(setter(each = "exclude_tag"))]
    pub excluded_tags: Vec<Tag>,
    pub excluded_tags_mode: Option<TagSearchMode>,
    #[builder(setter(each = "add_status"))]
    pub status: Vec<MangaStatus>,
    /// Languages the manga results are originally published in.
    #[builder(setter(each = "add_original_language"))]
    pub original_language: Vec<Language>,
    /// A list of original languages to exclude.
    #[builder(setter(each = "exclude_original_language"))]
    pub excluded_original_language: Vec<Language>,
    /// A list of languages that the manga is translated into.
    #[builder(setter(each = "add_available_translated_language"))]
    pub available_translated_language: Vec<Language>,
    #[builder(setter(each = "add_publication_demographic"))]
    pub publication_demographic: Vec<Demographic>,
    #[builder(setter(each = "add_manga_id"))]
    #[serde(rename = "ids")]
    pub manga_ids: Vec<Uuid>,
    #[builder(setter(each = "add_content_rating"))]
    pub content_rating: Vec<ContentRating>,
    /// DateTime string with following format: `YYYY-MM-DDTHH:MM:SS`.
    pub created_at_since: Option<MangaDexDateTime>,
    /// DateTime string with following format: `YYYY-MM-DDTHH:MM:SS`.
    pub updated_at_since: Option<MangaDexDateTime>,
    pub order: Option<MangaSortOrder>,
    #[builder(setter(each = "include"))]
    pub includes: Vec<ReferenceExpansionResource>,
    pub has_available_chapters: Option<bool>,
    /// Scanlation group ID.
    pub group: Option<Uuid>,
}

endpoint! {
    GET "/manga",
    #[query] ListManga,
    #[flatten_result] MangaListResponse
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
    use mangadex_api_types::{
        ContentRating, Demographic, Language, MangaDexDateTime, MangaStatus, ResponseType,
    };

    #[tokio::test]
    async fn list_manga_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
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
                        "state": "published",
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
            .and(path("/manga"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .manga()
            .search()
            .limit(1u32)
            .build()?
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
        assert!(manga.attributes.chapter_numbers_reset_on_new_volume);
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
    async fn list_manga_handles_400() -> anyhow::Result<()> {
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
            .and(path("/manga"))
            .respond_with(ResponseTemplate::new(400).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .manga()
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
