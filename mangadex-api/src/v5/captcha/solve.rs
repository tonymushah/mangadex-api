//! Builder for the CAPTCHA solve endpoint.
//!
//! <https://api.mangadex.org/swagger.html#/Captcha/post-captcha-solve>
//!
//! Captchas can be solved explicitly through this endpoint, another way is to add a
//! `X-Captcha-Result` header to any request.
//! The same logic will verify the captcha and is probably more convenient because it takes one less request.
//!
//! Authentication is optional.
//! Captchas are tracked for both the client IP and for the user ID, if you are logged in,
//! you want to send your session token but that is not required.
//!
//! # Examples
//!
//! ```rust
//! use mangadex_api::v5::MangaDexClient;
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! let captcha_res = client
//!     .captcha()
//!     .solve()
//!     .captcha_challenge("specialchallengetoken")
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("captcha solve: {:?}", captcha_res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;

use crate::HttpClientRef;
use mangadex_api_schema::NoData;
use mangadex_api_types::error::Result;

/// Mark a chapter as read for the current user.
///
/// Makes a request to `POST /captcha/solve`.
#[derive(Debug, Builder, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option))]
pub struct SolveCaptcha<'a> {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    pub(crate) http_client: HttpClientRef,

    pub captcha_challenge: &'a str,
}

endpoint! {
    POST "/captcha/solve",
    #[body] SolveCaptcha<'_>,
    #[discard_result] Result<NoData>
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use url::Url;
    use wiremock::matchers::{body_json, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::error::Error;

    #[tokio::test]
    async fn solve_captcha_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let expected_body = json!({
            "captchaChallenge": "solution",
        });
        let response_body = json!({"result": "ok"});

        Mock::given(method("POST"))
            .and(path(r"/captcha/solve"))
            .and(header("Content-Type", "application/json"))
            .and(body_json(expected_body))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        mangadex_client
            .captcha()
            .solve()
            .captcha_challenge("solution")
            .build()?
            .send()
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn solve_captcha_handles_400() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let expected_body = json!({
            "captchaChallenge": "solution",
        });
        let response_body = json!({
            "result": "error",
            "errors": []
        });

        Mock::given(method("POST"))
            .and(path(r"/captcha/solve"))
            .and(header("Content-Type", "application/json"))
            .and(body_json(expected_body))
            .respond_with(ResponseTemplate::new(400).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .captcha()
            .solve()
            .captcha_challenge("solution")
            .build()?
            .send()
            .await
            .expect_err("expected error");

        if let Error::Api(errors) = res {
            assert_eq!(errors.errors.len(), 0);
        }

        Ok(())
    }
}
