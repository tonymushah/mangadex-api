//! Builder for completing the account recovery process.
//!
//! <https://api.mangadex.org/swagger.html#/Account/post-account-recover-code>
//!
//! # Examples
//!
//! ```rust
//! use mangadex_api::MangaDexClient;
//! use mangadex_api_types::Password;
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! let account_complete_recovery_res = client
//!     .account()
//!     .complete_recovery()
//!     .code("abc123")
//!     .new_password(Password::parse("hunter23")?)
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("account recovery: {:?}", account_complete_recovery_res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;

use crate::HttpClientRef;
use mangadex_api_schema::NoData;
use mangadex_api_types::error::Result;
use mangadex_api_types::Password;

/// Complete an account recovery.
///
/// Makes a request to `POST /account/recover/{code}`.
#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
#[derive(Debug, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option))]
#[deprecated = "Usage deprecated after the introduction of OAuth authentification from Mangadex API 5.9"]
pub struct CompleteAccountRecovery {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub(crate) http_client: HttpClientRef,

    #[serde(skip_serializing)]
    pub code: String,

    /// Update the account's password to this value.
    ///
    /// Min: 8 characters
    ///
    /// Max: 1024 characters
    pub new_password: Password,
}

endpoint! {
    POST ("/account/recover/{}", code),
    #[body] CompleteAccountRecovery,
    #[discard_result] Result<NoData>
}

#[cfg(test)]
mod tests {
    use fake::faker::internet::en::Password;
    use fake::Fake;
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{body_json, header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::Password as MDPassword;

    #[tokio::test]
    async fn complete_recovery_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let new_password: String = Password(8..1024).fake();

        Mock::given(method("POST"))
            .and(path_regex(r"/account/recover/[0-9a-fA-F-]+"))
            .and(header("Content-Type", "application/json"))
            .and(body_json(json!({ "newPassword": new_password })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"result": "ok"})))
            .expect(1)
            .mount(&mock_server)
            .await;

        let code = Uuid::new_v4();

        let _ = mangadex_client
            .account()
            .complete_recovery()
            .code(code.to_string())
            .new_password(MDPassword::parse(&new_password)?)
            .build()?
            .send()
            .await?;

        Ok(())
    }
}
