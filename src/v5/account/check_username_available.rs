//! Builder for the username availability endpoint.
//!
//! <https://api.mangadex.org/swagger.html#/Account/get-account-available>
//!
//! # Examples
//!
//! ```rust
//! use mangadex_api::MangaDexClient;
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! let res = client
//!     .account()
//!     .check_username_available()
//!     .username("myusername")
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("username available: {:?}", res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use mangadex_api_schema::v5::CheckUsernameAvailableResponse;
use serde::Serialize;

use crate::HttpClientRef;

/// Check if the given username is available.
///
/// Makes a request to `GET /account/available`.
#[derive(Debug, Builder, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option))]
#[deprecated = "Usage deprecated after the introduction of OAuth authentification from Mangadex API 5.9"]
pub struct CheckUsernameAvailable<'a> {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    pub(crate) http_client: HttpClientRef,

    #[serde(skip)]
    pub username: &'a str,
}

endpoint! {
    GET "/account/available",
    #[query] CheckUsernameAvailable<'_>,
    CheckUsernameAvailableResponse
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use url::Url;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::{HttpClient, MangaDexClient};

    #[tokio::test]
    async fn check_username_available_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let response_body = json!({
            "available": true
        });

        Mock::given(method("GET"))
            .and(path(r"/account/available"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .account()
            .check_username_available()
            .username("myusername")
            .build()?
            .send()
            .await?;

        assert!(res.available);

        Ok(())
    }
}
