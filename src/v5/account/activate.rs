//! Builder for the account-activation endpoint.
//!
//! <https://api.mangadex.org/swagger.html#/Account/get-account-activate-code>
//!
//! # Examples
//!
//! ```rust
//! use mangadex_api::v5::MangaDexClient;
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! let account_activate_res = client
//!     .account()
//!     .activate()
//!     .code("abc123")
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("account activate: {:?}", account_activate_res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use mangadex_api_schema::NoData;
use serde::Serialize;

use crate::HttpClientRef;
use mangadex_api_types::error::Result;

/// Activate an account.
///
/// Makes a request to `POST /account/activate/{code}`.
#[derive(Debug, Builder, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option))]
pub struct ActivateAccount<'a> {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    pub(crate) http_client: HttpClientRef,

    #[serde(skip)]
    pub code: &'a str,
}

endpoint! {
    POST ("/account/activate/{}", code),
    #[no_data] ActivateAccount<'_>,
    #[discard_result] Result<NoData>
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
    async fn activate_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        Mock::given(method("POST"))
            .and(path_regex(r"/account/activate/[0-9a-fA-F-]+"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"result": "ok"})))
            .expect(1)
            .mount(&mock_server)
            .await;

        let code = Uuid::new_v4();

        let _ = mangadex_client
            .account()
            .activate()
            .code(code.to_string().as_str())
            .build()?
            .send()
            .await?;

        Ok(())
    }
}
