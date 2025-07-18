//! Builder for starting an upload session.
//!
//! <https://api.mangadex.org/docs/swagger.html#/Upload/begin-upload-session>
//!
//! ```rust
//! use uuid::Uuid;
//!
//! use mangadex_api::MangaDexClient;
//! // use mangadex_api_types::{Password, Username};
//! use mangadex_api_types::Language;
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! /*
//!
//!     let _login_res = client
//!         .auth()
//!         .login()
//!         .post()
//!         .username(Username::parse("myusername")?)
//!         .password(Password::parse("hunter23")?)
//!         .send()
//!         .await?;
//!
//!  */
//!
//! let group_id = Uuid::new_v4();
//! let manga_id = Uuid::new_v4();
//! let res = client
//!     .upload()
//!     .begin()
//!     .post()
//!     .add_group_id(&group_id)
//!     .manga_id(manga_id)
//!     .send()
//!     .await?;
//!
//! println!("session start: {:?}", res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use mangadex_api_schema::v5::UploadSessionData;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;

/// Start an upload session.
///
/// This requires authentication.
///
/// Makes a request to `POST /upload/begin`.
#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
#[derive(Debug, Builder, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[builder(
    setter(into, strip_option),
    build_fn(error = "crate::error::BuilderError")
)]
#[non_exhaustive]
pub struct StartUploadSession {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub http_client: HttpClientRef,

    #[builder(setter(each = "add_group_id"))]
    #[builder(default)]
    pub groups: Vec<Uuid>,
    #[serde(rename = "manga")]
    pub manga_id: Uuid,
}

endpoint! {
    POST "/upload/begin",
    #[body auth] StartUploadSession,
    #[rate_limited] UploadSessionData,
    StartUploadSessionBuilder
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
    use mangadex_api_types::{MangaDexDateTime, RelationshipType};

    #[tokio::test]
    async fn start_upload_session_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(non_exhaustive::non_exhaustive!(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            }))
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let group_id = Uuid::new_v4();
        let manga_id = Uuid::new_v4();
        let session_id = Uuid::new_v4();

        let datetime = MangaDexDateTime::new(&OffsetDateTime::now_utc());

        let expected_body = json!({
            "groups": [
                group_id
            ],
            "manga": manga_id
        });
        let response_body = json!({
            "result": "ok",
            "response": "entity",
            "data": {
                "id": session_id,
                "type": "upload_session",
                "attributes": {
                    "isCommitted": false,
                    "isProcessed": false,
                    "isDeleted": false,
                    "version": 1,
                    "createdAt": datetime.to_string(),
                    "updatedAt": datetime.to_string(),
                },
                "relationships": []
            }
        });

        Mock::given(method("POST"))
            .and(path("/upload/begin"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .and(header("Content-Type", "application/json"))
            .and(body_json(expected_body))
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

        let res = mangadex_client
            .upload()
            .begin()
            .post()
            .add_group_id(group_id)
            .manga_id(manga_id)
            .send()
            .await?;

        let res = &res.data;
        assert_eq!(res.id, session_id);
        assert_eq!(res.type_, RelationshipType::UploadSession);
        assert!(!res.attributes.is_committed);
        assert!(!res.attributes.is_processed);
        assert!(!res.attributes.is_deleted);
        assert_eq!(res.attributes.version, 1);
        assert_eq!(res.attributes.created_at.to_string(), datetime.to_string());
        assert_eq!(res.attributes.updated_at.to_string(), datetime.to_string());

        Ok(())
    }
}
