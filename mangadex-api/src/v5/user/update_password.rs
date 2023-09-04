//! Builder for updating a user password endpoint.
//!
//! <https://api.mangadex.org/swagger.html#/User/post-user-password>
//!
//! # Examples
//!
//! ```rust
//! use mangadex_api_types::{Password, Username};
//! use mangadex_api::v5::MangaDexClient;
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
//! let res = client
//!     .user()
//!     .update_password()
//!     .old_password(&Password::parse("hunter23")?)
//!     .new_password(&Password::parse("32retnuh")?)
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("update password: {:?}", res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;

use crate::v5::HttpClientRef;
use mangadex_api_schema::NoData;
use mangadex_api_types::error::Result;
use mangadex_api_types::Password;

/// Update a user password.
#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
#[derive(Debug, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), build_fn(error = "mangadex_api_types::error::BuilderError"))]
#[deprecated = "Usage deprecated after the introduction of OAuth authentification from Mangadex API 5.9"]
#[cfg(feature = "legacy-account")]
pub struct UpdateUserPassword {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub(crate) http_client: HttpClientRef,

    pub old_password: Password,
    pub new_password: Password,
}

endpoint! {
    POST "/user/password",
    #[body auth] UpdateUserPassword,
    #[discard_result] Result<NoData>
}

#[cfg(test)]
mod tests {
    use fake::faker::internet::en::Password;
    use fake::Fake;
    use serde_json::json;
    use url::Url;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::Password as MdPassword;

    #[tokio::test]
    async fn update_password_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let old_password: String = Password(8..1024).fake();
        let new_password: String = Password(8..1024).fake();
        let _expected_body = json!({
            "oldPassword": old_password,
            "newPassword": new_password
        });
        let response_body = json!({
            "result": "ok"
        });

        Mock::given(method("POST"))
            .and(path(r"/user/password"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .and(header("Content-Type", "application/json"))
            // TODO: Make the request body check work.
            // .and(body_json(expected_body))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        mangadex_client
            .user()
            .update_password()
            .old_password(MdPassword::parse(&old_password)?)
            .new_password(MdPassword::parse(&new_password)?)
            .build()?
            .send()
            .await?;

        Ok(())
    }
}
