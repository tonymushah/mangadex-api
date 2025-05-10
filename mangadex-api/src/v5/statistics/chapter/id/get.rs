//! Builder for getting a given chapter's statistics.
//!
//! <https://api.mangadex.org/docs/swagger.html#/Statistics/get-statistics-chapter-uuid>
//!
//! This only gets statistics for a single Manga.
//!
//! # Examples
//!
//! ```rust
//! use mangadex_api_types::MangaStatus;
//! use mangadex_api::v5::MangaDexClient;
//! use uuid::Uuid;
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! // Cool Tsuma no Saa-chan chapter 25
//! let chapter_id = Uuid::parse_str("1c3a8585-8df6-46d1-af98-fa777666abf2")?;
//!
//! let chapter_stats = client
//!     .statistics()
//!     .chapter()
//!     .id(chapter_id)
//!     .get()
//!     .send()
//!     .await?;
//!
//! println!("Response: {:?}", chapter_stats);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;
use mangadex_api_schema::v5::ChapterStatisticsResponse;

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
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
pub struct GetChapterStatistics {
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub http_client: HttpClientRef,

    pub chapter_id: Uuid,
}

endpoint! {
    GET ("/statistics/chapter/{}", chapter_id),
    // Known issue: Despite the API docs stating that authorization is required, the endpoint is
    // available to guests.
    #[no_data] GetChapterStatistics,
    #[flatten_result] ChapterStatisticsResponse,
    GetChapterStatisticsBuilder
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::{HttpClient, MangaDexClient};

    #[tokio::test]
    async fn find_chapter_statistics_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let manga_id = Uuid::new_v4();

        let thread_id = 4756728;
        let replies_count = 12;

        let response_body = json!({
            "result": "ok",
            "statistics": {
                manga_id.to_string(): {
                    "comments": {
                      "threadId": thread_id,
                      "repliesCount": replies_count
                    }
                }
            }
        });

        Mock::given(method("GET"))
            .and(path_regex("/statistics/chapter/[0-9a-fA-F-]+"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .statistics()
            .chapter()
            .id(manga_id)
            .get()
            .send()
            .await?;
        let ctt = res.statistics.get(&manga_id).ok_or(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "This id is not found",
        ))?;
        let comments = ctt.comments.ok_or(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "The comment is not found",
        ))?;
        assert_eq!(comments.thread_id, thread_id);
        assert_eq!(comments.replies_count, replies_count);
        Ok(())
    }
}
