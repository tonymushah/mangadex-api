//! Mark multiple chapters for one manga as read and/or unread.
//!
//! <https://api.mangadex.org/docs/swagger.html#/Chapter/post-manga-chapter-readmarkers>
//!
//! # Examples
//!
//! ```rust
//! use uuid::Uuid;
//!
//! use mangadex_api::MangaDexClient;
//! // use mangadex_api_types::{Password, Username};
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! /*
//!     // Put your login script here
//!     let _login_res = client
//!         .auth()
//!         .login()
//!         .username(Username::parse("myusername")?)
//!         .password(Password::parse("hunter23")?)
//!         .build()?
//!         .send()
//!         .await?;
//!
//!  */
//!
//! let manga_id = Uuid::new_v4();
//! let read_chapter_id = Uuid::new_v4();
//! let unread_chapter_id = Uuid::new_v4();
//! let res = client
//!     .manga()
//!     .id(manga_id)
//!     .read()
//!     .post()
//!     .mark_chapter_read(read_chapter_id)
//!     .mark_chapter_unread(unread_chapter_id)
//!     .send()
//!     .await?;
//!
//! println!("response: {:?}", res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;
use crate::Result;
use mangadex_api_schema::NoData;

/// Mark multiple manga chapters as read and/or unread for the current user.
///
/// Makes a request to `POST /manga/{id}/read`.
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
pub struct MarkChapterBatch {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub http_client: HttpClientRef,

    #[serde(skip_serializing)]
    pub manga_id: Uuid,
    #[builder(setter(each = "mark_chapter_read"), default)]
    pub chapter_ids_read: Vec<Uuid>,
    #[builder(setter(each = "mark_chapter_unread"), default)]
    pub chapter_ids_unread: Vec<Uuid>,
    #[serde(skip_serializing)]
    #[builder(default)]
    pub update_history: bool,
}

endpoint! {
    POST ("/manga/{}/read?updateHistory={}", manga_id, update_history),
    #[body auth] MarkChapterBatch,
    #[discard_result] Result<NoData>,
    MarkChapterBatchBuilder
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

    #[tokio::test]
    async fn mark_manga_chapters_read_fires_a_request_to_base_url() -> anyhow::Result<()> {
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
        let read_chapter_id = Uuid::new_v4();
        let unread_chapter_id = Uuid::new_v4();
        let expected_body = json!({
            "chapterIdsRead": [read_chapter_id],
            "chapterIdsUnread": [unread_chapter_id]
        });
        let response_body = json!({"result": "ok"});

        Mock::given(method("POST"))
            .and(path_regex(r"/manga/[0-9a-fA-F-]+/read"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .and(body_json(expected_body))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        mangadex_client
            .manga()
            .id(manga_id)
            .read()
            .post()
            .mark_chapter_read(read_chapter_id)
            .mark_chapter_unread(unread_chapter_id)
            .send()
            .await?;

        Ok(())
    }
}
