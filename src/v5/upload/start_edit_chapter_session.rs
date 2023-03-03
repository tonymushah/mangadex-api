//! Builder for starting an edit chapter session.
//!
//! <https://api.mangadex.org/swagger.html#/Upload/begin-edit-session>
//!
//! ```rust
//! use uuid::Uuid;
//!
//! use mangadex_api::MangaDexClient;
//! use mangadex_api::types::{Password, Username};
//! use mangadex_api::types::Language;
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
//! let chapter_id = Uuid::new_v4();
//! let res = client
//!     .upload()
//!     .start_edit_chapter_session()
//!     .chapter_id(&chapter_id)
//!     .version(2_u32)
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("edit chapter session start: {:?}", res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use mangadex_api_schema::v5::UploadSessionResponse;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;

/// Start an edit chapter session.
///
/// This requires authentication.
///
/// Makes a request to `POST /upload/begin/{id}`.
#[derive(Debug, Builder, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option))]
pub struct StartEditChapterSession<'a> {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    pub(crate) http_client: HttpClientRef,

    #[serde(skip)]
    pub chapter_id: &'a Uuid,

    pub version: u32,
}

endpoint! {
    POST ("/upload/begin/{}", chapter_id),
    #[body auth] StartEditChapterSession<'_>,
    UploadSessionResponse
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use time::OffsetDateTime;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{body_json, header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::{MangaDexDateTime, RelationshipType};

    #[tokio::test]
    async fn start_edit_chapter_session_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let chapter_id = Uuid::new_v4();
        let session_id = Uuid::new_v4();

        let datetime = MangaDexDateTime::new(&OffsetDateTime::now_utc());

        let expected_body = json!({
            "version": 2,
        });
        let response_body = json!({
            "id": session_id,
            "type": "upload_session",
            "attributes": {
                "isCommitted": false,
                "isProcessed": false,
                "isDeleted": false,
                "version": 2,
                "createdAt": datetime.to_string(),
                "updatedAt": datetime.to_string(),
            },
        });

        Mock::given(method("POST"))
            .and(path_regex("/upload/begin/[0-9a-fA-F-]+"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .and(header("Content-Type", "application/json"))
            .and(body_json(expected_body))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .upload()
            .start_edit_chapter_session()
            .chapter_id(&chapter_id)
            .version(2_u32)
            .build()?
            .send()
            .await?;

        assert_eq!(res.id, session_id);
        assert_eq!(res.type_, RelationshipType::UploadSession);
        assert!(!res.attributes.is_committed);
        assert!(!res.attributes.is_processed);
        assert!(!res.attributes.is_deleted);
        assert_eq!(res.attributes.version, 2);
        assert_eq!(res.attributes.created_at.to_string(), datetime.to_string());
        assert_eq!(res.attributes.updated_at.to_string(), datetime.to_string());

        Ok(())
    }
}
