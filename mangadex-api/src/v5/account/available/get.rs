//! Builder for the username availability endpoint.
//!
//! <https://api.mangadex.org/docs/swagger.html#/Account/get-account-available>
//! <https://api.mangadex.org/docs/redoc.html#tag/Account/operation/get-account-available>
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
//!     .available()
//!     .get()
//!     .username("myusername")
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
use mangadex_api_types::error::BuilderError;

/// Check if the given username is available.
///
/// Makes a request to `GET /account/available`.
#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
#[derive(Debug, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), build_fn(error = "BuilderError"))]
#[deprecated = "Usage deprecated after the introduction of OAuth authentification from Mangadex API 5.9"]
pub struct CheckUsernameAvailable {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub(crate) http_client: HttpClientRef,

    #[serde(skip_serializing)]
    pub username: String,
}

endpoint! {
    GET "/account/available",
    #[query] CheckUsernameAvailable,
    CheckUsernameAvailableResponse,
    CheckUsernameAvailableBuilder
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
            .available()
            .get()
            .username("myusername")
            .send()
            .await?;

        assert!(res.available);

        Ok(())
    }
}
