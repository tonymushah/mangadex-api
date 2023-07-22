//! Builder for the update manga reading status endpoint.
//!
//! <https://api.mangadex.org/swagger.html#/Manga/post-manga-id-status>
//!
//! # Examples
//!
//! ```rust
//! use uuid::Uuid;
//!
//! use mangadex_api_types::ReadingStatus;
//! use mangadex_api::v5::MangaDexClient;
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! let manga_id = Uuid::new_v4();
//! let res = client
//!     .manga()
//!     .update_reading_status()
//!     .manga_id(&manga_id)
//!     .status(Some(ReadingStatus::Reading))
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("status: {:?}", res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;
use mangadex_api_schema::NoData;
use mangadex_api_types::error::Result;
use mangadex_api_types::ReadingStatus;

#[derive(Debug, Deserialize, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into), pattern = "owned")]
pub struct UpdateMangaReadingStatus<'a> {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    pub(crate) http_client: HttpClientRef,

    #[serde(skip_serializing)]
    pub manga_id: &'a Uuid,

    /// Using a `None` (`null`) value will remove the reading status.
    pub status: Option<ReadingStatus>,
}

endpoint! {
    POST ("/manga/{}/status", manga_id),
    #[body auth] UpdateMangaReadingStatus<'_>,
    #[discard_result] Result<NoData>
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{body_json, header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::ReadingStatus;

    #[tokio::test]
    async fn update_manga_reading_status_fires_a_request_to_base_url() -> anyhow::Result<()> {
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
        let expected_body = json!({
            "status": "reading",
        });
        let response_body = json!({
            "result": "ok",
        });

        Mock::given(method("POST"))
            .and(path_regex(r"/manga/[0-9a-fA-F-]+/status"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .and(header("Content-Type", "application/json"))
            .and(body_json(expected_body))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        mangadex_client
            .manga()
            .update_reading_status()
            .manga_id(&manga_id)
            .status(Some(ReadingStatus::Reading))
            .build()?
            .send()
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn update_manga_reading_status_sets_status_to_null() -> anyhow::Result<()> {
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
        let expected_body = json!({
            "status": null,
        });
        let response_body = json!({
            "result": "ok",
        });

        Mock::given(method("POST"))
            .and(path_regex(r"/manga/[0-9a-fA-F-]+/status"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .and(header("Content-Type", "application/json"))
            .and(body_json(expected_body))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        mangadex_client
            .manga()
            .update_reading_status()
            .manga_id(&manga_id)
            .status(None)
            .build()?
            .send()
            .await?;

        Ok(())
    }
}
