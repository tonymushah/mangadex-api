//! Builder for the manga feed endpoint to get a list of new chapters for a manga.
//!
//! <https://api.mangadex.org/docs/swagger.html#/Manga/get-manga-id-feed>
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
//! let manga_id = Uuid::new_v4();
//! let manga_res = client
//!     .manga()
//!     .id(manga_id)
//!     .feed()
//!     .get()
//!     .send()
//!     .await?;
//!
//! println!("Manga feed: {:?}", manga_res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;
use mangadex_api_schema::v5::ChapterListResponse;
use mangadex_api_types::{
    ContentRating, IncludeExternalUrl, IncludeFuturePages, IncludeFuturePublishAt,
    IncludeFutureUpdates, Language, MangaDexDateTime, MangaFeedSortOrder,
    ReferenceExpansionResource,
};

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
#[non_exhaustive]
pub struct GetMangaFeed {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub http_client: HttpClientRef,

    #[serde(skip_serializing)]
    pub manga_id: Uuid,

    // `manga_id` cannot use the `Default` trait so these attributes have to be manually set.
    #[builder(default)]
    pub limit: Option<u32>,
    #[builder(default)]
    pub offset: Option<u32>,
    #[builder(setter(each = "add_translated_language"), default)]
    pub translated_language: Vec<Language>,
    #[builder(setter(each = "add_original_language"), default)]
    pub original_language: Vec<Language>,
    #[builder(setter(each = "exclude_original_language"), default)]
    pub excluded_original_language: Vec<Language>,
    #[builder(setter(each = "add_content_rating"), default)]
    pub content_rating: Vec<ContentRating>,
    /// Groups to exclude from the results.
    #[builder(setter(each = "excluded_group"), default)]
    pub excluded_groups: Vec<Uuid>,
    /// Uploaders to exclude from the results.
    #[builder(setter(each = "excluded_uploader"), default)]
    pub excluded_uploaders: Vec<Uuid>,
    /// Flag to include future chapter updates in the results.
    ///
    /// Default: `IncludeFutureUpdates::Include` (1)
    #[builder(default)]
    pub include_future_updates: Option<IncludeFutureUpdates>,
    /// DateTime string with following format: `YYYY-MM-DDTHH:MM:SS`.
    #[builder(default)]
    pub created_at_since: Option<MangaDexDateTime>,
    /// DateTime string with following format: `YYYY-MM-DDTHH:MM:SS`.
    #[builder(default)]
    pub updated_at_since: Option<MangaDexDateTime>,
    /// DateTime string with following format: `YYYY-MM-DDTHH:MM:SS`.
    #[builder(default)]
    pub publish_at_since: Option<MangaDexDateTime>,
    #[builder(default)]
    pub order: Option<MangaFeedSortOrder>,
    #[builder(setter(each = "include"), default)]
    pub includes: Vec<ReferenceExpansionResource>,
    #[builder(default)]
    pub include_empty_pages: Option<IncludeFuturePages>,
    #[builder(default)]
    pub include_future_publish_at: Option<IncludeFuturePublishAt>,
    #[builder(default)]
    pub include_external_url: Option<IncludeExternalUrl>,
}

endpoint! {
    GET ("/manga/{}/feed", manga_id),
    #[query] GetMangaFeed,
    #[flatten_result] ChapterListResponse,
    GetMangaFeedBuilder
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use time::OffsetDateTime;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::MangaDexDateTime;

    #[tokio::test]
    async fn get_manga_feed_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let manga_id = Uuid::new_v4();
        let chapter_id = Uuid::new_v4();
        let uploader_id = Uuid::new_v4();

        let datetime = MangaDexDateTime::new(&OffsetDateTime::now_utc());

        let response_body = json!({
            "result": "ok",
            "response": "collection",
            "data": [
                {
                    "id": chapter_id,
                    "type": "chapter",
                    "attributes": {
                        "title": "Chapter title",
                        "volume": null,
                        "chapter": "1",
                        "pages": 4,
                        "translatedLanguage": "en",
                        "hash": "123456abcdef",
                        "data": [
                            "1.jpg"
                        ],
                        "dataSaver": [
                            "1.jpg"
                        ],
                        "uploader": uploader_id,
                        "version": 1,
                        "createdAt": datetime.to_string(),
                        "updatedAt": datetime.to_string(),
                        "publishAt": datetime.to_string(),
                        "readableAt": datetime.to_string(),
                    },
                    "relationships": []
                }
            ],
            "limit": 1,
            "offset": 0,
            "total": 1
        });

        Mock::given(method("GET"))
            .and(path_regex(r"/manga/[0-9a-fA-F-]+/feed"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _ = mangadex_client
            .manga()
            .id(manga_id)
            .feed()
            .get()
            .limit(1u32)
            .send()
            .await?;

        Ok(())
    }
}
