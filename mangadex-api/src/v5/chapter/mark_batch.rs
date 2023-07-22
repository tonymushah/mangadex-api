//! Mark multiple chapters for one manga as read and/or unread.
//!
//! <https://api.mangadex.org/swagger.html#/Chapter/post-manga-chapter-readmarkers>
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
//! let read_chapter_id = Uuid::new_v4();
//! let unread_chapter_id = Uuid::new_v4();
//! let res = client
//!     .chapter()
//!     .mark_batch()
//!     .manga_id(&manga_id)
//!     .mark_chapter_read(&read_chapter_id)
//!     .mark_chapter_unread(&unread_chapter_id)
//!     .build()?
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
use mangadex_api_schema::NoData;
use mangadex_api_types::error::Result;

/// Mark multiple manga chapters as read and/or unread for the current user.
///
/// Makes a request to `POST /manga/{id}/read`.
#[derive(Debug, Builder, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option))]
pub struct MarkChapterBatch<'a> {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    pub(crate) http_client: HttpClientRef,

    #[serde(skip_serializing)]
    pub manga_id: &'a Uuid,
    #[builder(setter(each = "mark_chapter_read"), default)]
    pub chapter_ids_read: Vec<&'a Uuid>,
    #[builder(setter(each = "mark_chapter_unread"), default)]
    pub chapter_ids_unread: Vec<&'a Uuid>,
}

endpoint! {
    POST ("/manga/{}/read", manga_id),
    #[body auth] MarkChapterBatch<'_>,
    #[discard_result] Result<NoData>
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};

    #[tokio::test]
    async fn mark_manga_chapters_read_fires_a_request_to_base_url() -> anyhow::Result<()> {
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
        let read_chapter_id = Uuid::new_v4();
        let unread_chapter_id = Uuid::new_v4();
        let _expected_body = json!({
            "chapterIdsRead": [read_chapter_id],
            "chapterIdsUnread": [unread_chapter_id]
        });
        let response_body = json!({"result": "ok"});

        Mock::given(method("POST"))
            .and(path_regex(r"/manga/[0-9a-fA-F-]+/read"))
            .and(header("Authorization", "Bearer sessiontoken"))
            // TODO: Make the request body check work.
            // .and(body_json(expected_body))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        mangadex_client
            .chapter()
            .mark_batch()
            .manga_id(&manga_id)
            .mark_chapter_read(&read_chapter_id)
            .mark_chapter_unread(&unread_chapter_id)
            .build()?
            .send()
            .await?;

        Ok(())
    }
}
