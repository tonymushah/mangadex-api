//! Builder for the account-creation endpoint.
//!
//! <https://api.mangadex.org/swagger.html#/Account/post-account-create>
//!
//! # Examples
//!
//! ```rust
//! use mangadex_api::v5::{MangaDexClient};
//! use mangadex_api::types::{Password, Username};
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! let account_create_res = client
//!     .account()
//!     .create()
//!     .username(Username::parse("myusername")?)
//!     .password(Password::parse("hunter2")?)
//!     .email("test@example.com")
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("account create: {:?}", account_create_res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;

use crate::v5::HttpClientRef;
use mangadex_api_schema::v5::UserResponse;
use mangadex_api_types::{Password, Username};

/// Create a new account.
///
/// Makes a request to `POST /account/create`.
#[derive(Debug, Builder, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option))]
#[deprecated]
pub struct CreateAccount<'a> {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    pub(crate) http_client: HttpClientRef,

    /// Unique username, used for logging in.
    ///
    /// Min: 1 character
    ///
    /// Max: 64 characters
    pub username: Username,

    /// Set the account's password.
    ///
    /// Min: 8 characters
    ///
    /// Max: 1024 characters
    pub password: Password,

    /// E-mail address; used for notifications and account recovery from MangaDex.
    pub email: &'a str,
}

endpoint! {
    POST "/account/create",
    #[body] CreateAccount<'_>,
    #[flatten_result] UserResponse
}

#[cfg(test)]
mod tests {
    use fake::faker::internet::en::{Password, SafeEmail};
    use fake::faker::lorem::en::Word;
    use fake::Fake;
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::{Password as MDPassword, Username};

    #[tokio::test]
    async fn create_account_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let username: String = Word().fake();
        let password: String = Password(8..1024).fake();
        let email: String = SafeEmail().fake();

        let _expected_body = json!({
            "username": username,
            "password": password,
            "email": email
        });
        let response_body = json!({
            "result": "ok",
            "response": "entity",
            "data": {
                "id": Uuid::new_v4(),
                "type": "user",
                "attributes": {
                    "username": username,
                    "roles": [
                        "ROLE_MEMBER",
                        "ROLE_GROUP_MEMBER",
                        "ROLE_GROUP_LEADER"
                    ],
                    "version": 1
                },
                "relationships": [
                    {
                        "id": "a3219a4f-73c0-4213-8730-05985130539a",
                        "type": "scanlation_group"
                    }
                ]
            }
        });

        Mock::given(method("POST"))
            .and(path("/account/create"))
            .and(header("Content-Type", "application/json"))
            // TODO: Make the request body check work with multiple fields.
            // .and(body_json(expected_body))
            .respond_with(ResponseTemplate::new(201).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _ = mangadex_client
            .account()
            .create()
            .username(Username::parse(&username)?)
            .password(MDPassword::parse(&password)?)
            .email(email.as_str())
            .build()?
            .send()
            .await?;

        Ok(())
    }
}
