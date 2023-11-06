//! Builder for resending the account activation code.
//!
//! <https://api.mangadex.org/docs/swagger.html#/Account/post-account-activate-resend>
//! <https://api.mangadex.org/docs/redoc.html#tag/Account/operation/post-account-activate-resend>
//!
//! # Examples
//!
//! ```rust
//! use mangadex_api::v5::MangaDexClient;
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! let account_resend_res = client
//!     .account()
//!     .activate()
//!     .resend()
//!     .post()
//!     .email("test@example.com")
//!     .send()
//!     .await?;
//!
//! println!("account resend activation code: {:?}", account_resend_res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;

use crate::HttpClientRef;
use mangadex_api_schema::NoData;
use mangadex_api_types::error::{BuilderError, Result};

/// Resend the account activation code.
///
/// Makes a request to `POST /account/activate/resend`.
#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
#[derive(Debug, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), build_fn(error = "BuilderError"))]
#[deprecated = "Usage deprecated after the introduction of OAuth authentification from Mangadex API 5.9"]
pub struct ResendActivationCode {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub(crate) http_client: HttpClientRef,

    pub email: String,
}

endpoint! {
    POST "/account/activate/resend",
    #[body] ResendActivationCode,
    #[discard_result] Result<NoData>,
    ResendActivationCodeBuilder
}

#[cfg(test)]
mod tests {
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;
    use serde_json::json;
    use url::Url;
    use wiremock::matchers::{body_json, header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::{HttpClient, MangaDexClient};

    #[tokio::test]
    async fn resend_activation_code_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let email: String = SafeEmail().fake();

        Mock::given(method("POST"))
            .and(path_regex(r"/account/activate/resend"))
            .and(header("Content-Type", "application/json"))
            .and(body_json(json!({ "email": email })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"result": "ok"})))
            .expect(1)
            .mount(&mock_server)
            .await;

        mangadex_client
            .account()
            .activate()
            .resend()
            .post()
            .email(email)
            .send()
            .await?;

        Ok(())
    }
}
