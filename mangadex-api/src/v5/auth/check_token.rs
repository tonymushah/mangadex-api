//! Builder for the auth session check endpoint.
//!
//! <https://api.mangadex.org/swagger.html#/Auth/get-auth-check>
//!
//! # Examples
//!
//! ```rust
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
//! let auth_check_res = client
//!     .auth()
//!     .check_token()
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("auth check: {:?}", auth_check_res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;

use crate::HttpClientRef;
use mangadex_api_schema::v5::CheckTokenResponse;
use mangadex_api_types::error::Result;

/// Check the session token and get additional user information.
///
/// Makes a request to `POST /auth/check`.
// It doesn't make much sense to make this a builder pattern but for consistency, it is.
#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
#[derive(Debug, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), build_fn(error = "mangadex_api_types::error::BuilderError"))]
pub struct CheckToken {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub(crate) http_client: HttpClientRef,
}

endpoint! {
    GET "/auth/check",
    #[no_data auth] CheckToken,
    #[flatten_result] Result<CheckTokenResponse>
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use url::Url;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};

    #[tokio::test]
    async fn check_token_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let response_body = json!({
            "result": "ok",
            "isAuthenticated": true,
            "roles": [
                "ROLE_MEMBER",
                "IS_JWT_AUTHENTICATED",
                "IS_AUTHENTICATED_FULLY",
                "IS_AUTHENTICATED_ANONYMOUSLY",
                "IS_AUTHENTICATED_REMEMBERED"
            ],
            "permissions": [
                "user.list",
                "manga.view",
                "chapter.view",
                "author.view",
                "scanlation_group.view",
                "cover.view",
                "manga.list",
                "chapter.list",
                "author.list",
                "scanlation_group.list",
                "cover.list"
            ]
        });

        Mock::given(method("GET"))
            .and(path(r"/auth/check"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _ = mangadex_client.auth().check_token().build()?.send().await?;

        Ok(())
    }
}
