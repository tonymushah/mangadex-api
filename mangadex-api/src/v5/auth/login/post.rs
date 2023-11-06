//! Builder for the auth login endpoint.
//!
//! This does not support 2-factor authentication currently.
//!
//! <https://api.mangadex.org/docs/swagger.html#/Auth/post-auth-login>
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
//! let login_res = client
//!     .auth()
//!     .login()
//!     .post()
//!     .username(Username::parse("myusername")?)
//!     .password(Password::parse("hunter2")?)
//!     .send()
//!     .await?;
//!
//! println!("login: {:?}", login_res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use mangadex_api_schema::Limited;
use serde::Serialize;

use crate::v5::HttpClientRef;
use mangadex_api_schema::v5::LoginResponse;
use mangadex_api_types::error::Result;
use mangadex_api_types::{Password, Username};

/// Log into an account.
///
/// Makes a request to `POST /auth/login`.
#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
#[derive(Debug, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(
    setter(into, strip_option),
    build_fn(error = "mangadex_api_types::error::BuilderError")
)]
#[deprecated = "Usage deprecated after the introduction of OAuth authentification from Mangadex API 5.9"]
pub struct Login {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub(crate) http_client: HttpClientRef,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub username: Option<Username>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub email: Option<String>,

    pub password: Password,
}

impl Login {
    pub async fn send(&self) -> Result<Limited<LoginResponse>> {
        #[cfg(all(
            not(feature = "multi-thread"),
            not(feature = "tokio-multi-thread"),
            not(feature = "rw-multi-thread")
        ))]
        let res = {
            let res = self
                .http_client
                .try_borrow()?
                .send_request_with_rate_limit(self)
                .await?;

            self.http_client
                .borrow_mut()
                .set_auth_tokens(&res.body.token);

            res
        };
        #[cfg(any(feature = "multi-thread", feature = "tokio-multi-thread"))]
        let res = {
            let res = self
                .http_client
                .lock()
                .await
                .send_request_with_rate_limit(self)
                .await?;

            self.http_client
                .lock()
                .await
                .set_auth_tokens(&res.body.token);

            res
        };

        #[cfg(feature = "rw-multi-thread")]
        let res = {
            let res = self
                .http_client
                .read()
                .await
                .send_request_with_rate_limit(self)
                .await?;

            self.http_client
                .write()
                .await
                .set_auth_tokens(&res.body.token);

            res
        };

        Ok(res)
    }
}

endpoint! {
    POST "/auth/login",
    #[body] Login,
    #[no_send] LoginResponse
}

builder_send! {
    #[builder] LoginBuilder,
    Limited<LoginResponse>
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use url::Url;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::error::Error;
    use mangadex_api_types::{Password, Username};

    #[tokio::test]
    async fn login_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let _expected_body = json!({
            "username": "myusername",
            "password": "mypassword"
        });
        let response_body = json!({
            "result": "ok",
            "token": {
                "session": "sessiontoken",
                "refresh": "refreshtoken"
            }
        });

        Mock::given(method("POST"))
            .and(path(r"/auth/login"))
            .and(header("Content-Type", "application/json"))
            // TODO: Make the request body check work.
            // .and(body_json(expected_body))
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

        let _ = mangadex_client
            .auth()
            .login()
            .post()
            .username(Username::parse("myusername")?)
            .password(Password::parse("mypassword")?)
            .send()
            .await?;

        #[cfg(all(
            not(feature = "multi-thread"),
            not(feature = "tokio-multi-thread"),
            not(feature = "rw-multi-thread")
        ))]
        assert_eq!(
            mangadex_client.http_client.try_borrow()?.get_tokens(),
            Some(&AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
        );
        #[cfg(any(feature = "multi-thread", feature = "tokio-multi-thread"))]
        assert_eq!(
            mangadex_client.http_client.lock().await.get_tokens(),
            Some(&AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
        );
        #[cfg(feature = "rw-multi-thread")]
        assert_eq!(
            mangadex_client.http_client.read().await.get_tokens(),
            Some(&AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
        );

        Ok(())
    }

    #[tokio::test]
    async fn logout_handles_400() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let _expected_body = json!({
            "username": "myusername",
            "password": "mypassword"
        });
        let response_body = json!({
            "result": "error",
            "errors": []
        });

        Mock::given(method("POST"))
            .and(path(r"/auth/login"))
            .and(header("Content-Type", "application/json"))
            // TODO: Make the request body check work.
            // .and(body_json(expected_body))
            .respond_with(
                ResponseTemplate::new(400)
                    .insert_header("x-ratelimit-retry-after", "1698723860")
                    .insert_header("x-ratelimit-limit", "40")
                    .insert_header("x-ratelimit-remaining", "39")
                    .set_body_json(response_body),
            )
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .auth()
            .login()
            .post()
            .username(Username::parse("myusername")?)
            .password(Password::parse("mypassword")?)
            .send()
            .await
            .expect_err("expected error");

        #[cfg(all(
            not(feature = "multi-thread"),
            not(feature = "tokio-multi-thread"),
            not(feature = "rw-multi-thread")
        ))]
        assert_eq!(mangadex_client.http_client.try_borrow()?.get_tokens(), None);
        #[cfg(any(feature = "multi-thread", feature = "tokio-multi-thread"))]
        assert_eq!(mangadex_client.http_client.lock().await.get_tokens(), None);
        #[cfg(feature = "rw-multi-thread")]
        assert_eq!(mangadex_client.http_client.read().await.get_tokens(), None);

        if let Error::Api(errors) = res {
            assert_eq!(errors.errors.len(), 0);
        }

        Ok(())
    }

    #[tokio::test]
    async fn logout_handles_401() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let _expected_body = json!({
            "username": "myusername",
            "password": "mypassword"
        });

        Mock::given(method("POST"))
            .and(path(r"/auth/login"))
            .and(header("Content-Type", "application/json"))
            // TODO: Make the request body check work.
            // .and(body_json(expected_body))
            .respond_with(
                ResponseTemplate::new(401)
                    .insert_header("x-ratelimit-retry-after", "1698723860")
                    .insert_header("x-ratelimit-limit", "40")
                    .insert_header("x-ratelimit-remaining", "39"),
            )
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .auth()
            .login()
            .post()
            .username(Username::parse("myusername")?)
            .password(Password::parse("mypassword")?)
            .send()
            .await
            .expect_err("expected error");

        #[cfg(all(
            not(feature = "multi-thread"),
            not(feature = "tokio-multi-thread"),
            not(feature = "rw-multi-thread")
        ))]
        assert_eq!(mangadex_client.http_client.try_borrow()?.get_tokens(), None);
        #[cfg(any(feature = "multi-thread", feature = "tokio-multi-thread"))]
        assert_eq!(mangadex_client.http_client.lock().await.get_tokens(), None);
        #[cfg(feature = "rw-multi-thread")]
        assert_eq!(mangadex_client.http_client.read().await.get_tokens(), None);

        match res {
            Error::RequestError(_) => {}
            _ => panic!("unexpected error"),
        }

        Ok(())
    }

    #[tokio::test]
    async fn logout_handles_http_503() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let _expected_body = json!({
            "username": "myusername",
            "password": "mypassword"
        });

        Mock::given(method("POST"))
            .and(path(r"/auth/login"))
            .and(header("Content-Type", "application/json"))
            // TODO: Make the request body check work.
            // .and(body_json(expected_body))
            .respond_with(
                ResponseTemplate::new(503)
                    .insert_header("x-ratelimit-retry-after", "1698723860")
                    .insert_header("x-ratelimit-limit", "40")
                    .insert_header("x-ratelimit-remaining", "39"),
            )
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .auth()
            .login()
            .post()
            .username(Username::parse("myusername")?)
            .password(Password::parse("mypassword")?)
            .send()
            .await
            .expect_err("expected error");

        #[cfg(all(
            not(feature = "multi-thread"),
            not(feature = "tokio-multi-thread"),
            not(feature = "rw-multi-thread")
        ))]
        assert_eq!(mangadex_client.http_client.try_borrow()?.get_tokens(), None);
        #[cfg(any(feature = "multi-thread", feature = "tokio-multi-thread"))]
        assert_eq!(mangadex_client.http_client.lock().await.get_tokens(), None);
        #[cfg(feature = "rw-multi-thread")]
        assert_eq!(mangadex_client.http_client.read().await.get_tokens(), None);

        match res {
            Error::ServerError(..) => {}
            _ => panic!("unexpected error"),
        }

        Ok(())
    }
}
