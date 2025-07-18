//! Builder for checking if a given manga / locale for a User needs moderation approval.
//!
//! <https://api.mangadex.org/docs/swagger.html#/Upload/upload-check-approval-required>
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
//! let manga_id = Uuid::new_v4();
//! let res = client
//!     .upload()
//!     .check_approval_required()
//!     .post()
//!     .manga_id(manga_id)
//!     .locale(Language::English)
//!     .send()
//!     .await?;
//!
//! println!("session start: {:?}", res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use mangadex_api_schema::v5::upload_required_approval::UploadRequiredApproval;
use mangadex_api_types::Language;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;

/// Check if a given manga / locale for a User needs moderation approval.
///
/// This requires authentication.
///
/// Makes a request to `POST /upload/check-approval-required`.
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
pub struct CheckApprovalRequired {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub http_client: HttpClientRef,

    #[serde(rename = "manga")]
    pub manga_id: Uuid,
    pub locale: Language,
}

endpoint! {
    POST "/upload/check-approval-required",
    #[body auth] CheckApprovalRequired,
    #[rate_limited] UploadRequiredApproval,
    CheckApprovalRequiredBuilder
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
    use mangadex_api_types::Language;

    use serde::Serialize;

    #[derive(Clone, Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct ExceptedBody {
        manga: Uuid,
        locale: Language,
    }

    #[tokio::test]
    async fn check_approval_required_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(non_exhaustive::non_exhaustive!(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            }))
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let manga = Uuid::new_v4();
        let locale = Language::English;

        let expected_body = ExceptedBody { manga, locale };

        let response_body = json!({
            "result": "ok",
            "requiresApproval": false
        });
        Mock::given(method("POST"))
            .and(path_regex(r"/upload/check-approval-required"))
            .and(header("authorization", "Bearer sessiontoken"))
            .and(header("content-type", "application/json"))
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
            .check_approval_required()
            .post()
            .locale(locale)
            .manga_id(manga)
            .send()
            .await?;

        let res = res.body;

        assert!(!res.requires_approval.unwrap());

        Ok(())
    }
    #[tokio::test]
    async fn check_approval_required_parses_404_not_found() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(non_exhaustive::non_exhaustive!(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            }))
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let manga = Uuid::new_v4();
        let locale = Language::English;

        let expected_body = ExceptedBody { manga, locale };

        let response_body = json!({
            "result": "ok"
        });
        Mock::given(method("POST"))
            .and(path_regex(r"/upload/check-approval-required"))
            .and(header("authorization", "Bearer sessiontoken"))
            .and(header("content-type", "application/json"))
            .and(body_json(expected_body))
            .respond_with(
                ResponseTemplate::new(404)
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
            .check_approval_required()
            .post()
            .locale(locale)
            .manga_id(manga)
            .send()
            .await?;

        let res = res.body;

        assert!(res.is_manga_not_found());

        Ok(())
    }
}
