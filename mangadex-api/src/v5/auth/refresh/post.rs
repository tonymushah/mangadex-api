//! Builder for the auth session token refresh endpoint.
//!
//! <https://api.mangadex.org/swagger.html#/Auth/post-auth-refresh>
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
//! // Logging in will store the refresh token in the `MangaDexClient`.
//! let _login_res = client
//!     .auth()
//!     .login()
//!     .username(Username::parse("myusername")?)
//!     .password(Password::parse("hunter23")?)
//!     .build()?
//!     .send()
//!     .await?;
//!
//! let refresh_res = client
//!     .auth()
//!     .refresh_token()
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("refresh: {:?}", refresh_res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;

use crate::HttpClientRef;
use mangadex_api_schema::v5::RefreshTokenResponse;
use mangadex_api_types::error::{Error, Result};

/// Get a new session and refresh token.
///
/// Makes a request to `POST /auth/refresh`.
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
pub struct RefreshToken {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub(crate) http_client: HttpClientRef,

    /// Refresh token.
    ///
    /// If this is not provided, the refresh token associated with the logged in user will be used.
    #[serde(rename = "token")]
    #[builder(default)]
    pub refresh_token: String,
}

impl RefreshToken {
    pub async fn send(&mut self) -> Result<RefreshTokenResponse> {
        // Attempt to get the authenticated user's refresh token, otherwise return an error.
        if self.refresh_token.trim().is_empty() {
            #[cfg(not(feature = "multi-thread"))]
            let http_client = &self.http_client.try_borrow()?;
            #[cfg(feature = "multi-thread")]
            let http_client = &self.http_client.lock().await;

            let refresh_token = &http_client
                .get_tokens()
                .ok_or(Error::MissingTokens)?
                .refresh;
            self.refresh_token = refresh_token.clone();
        }

        #[cfg(not(feature = "multi-thread"))]
        {
            let res = self.http_client.try_borrow()?.send_request(self).await??;

            self.http_client
                .try_borrow_mut()?
                .set_auth_tokens(&res.token);

            Ok(res)
        }
        #[cfg(feature = "multi-thread")]
        {
            let res = self.http_client.lock().await.send_request(self).await??;

            self.http_client.lock().await.set_auth_tokens(&res.token);

            Ok(res)
        }
    }
}

endpoint! {
    POST "/auth/refresh",
    #[body] RefreshToken,
    #[no_send] Result<RefreshTokenResponse>
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{body_json, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::error::Error;

    #[tokio::test]
    async fn refresh_token_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let expected_body = json!({"token": "refreshtoken"});
        let response_body = json!({
            "result": "ok",
            "token": {
                "session": "newsessiontoken",
                "refresh": "newrefreshtoken"
            },
            "message": "Token refreshed!"
        });

        Mock::given(method("POST"))
            .and(path(r"/auth/refresh"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .and(header("Content-Type", "application/json"))
            .and(body_json(expected_body))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _ = mangadex_client
            .auth()
            .refresh()
            .post()
            .build()?
            .send()
            .await?;

        #[cfg(not(feature = "multi-thread"))]
        assert_eq!(
            mangadex_client.http_client.try_borrow()?.get_tokens(),
            Some(&AuthTokens {
                session: "newsessiontoken".to_string(),
                refresh: "newrefreshtoken".to_string(),
            })
        );
        #[cfg(feature = "multi-thread")]
        assert_eq!(
            mangadex_client.http_client.lock().await.get_tokens(),
            Some(&AuthTokens {
                session: "newsessiontoken".to_string(),
                refresh: "newrefreshtoken".to_string(),
            })
        );

        Ok(())
    }

    #[tokio::test]
    async fn refresh_handles_400() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let error_id = Uuid::new_v4();

        let expected_body = json!({"token": ""});
        let response_body = json!({
            "result": "error",
            "errors": [{
                "id": error_id.to_string(),
                "status": 400,
                "title": "Empty token",
                "detail": "Token cannot be empty"
            }]
        });

        Mock::given(method("POST"))
            .and(path(r"/auth/refresh"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .and(header("Content-Type", "application/json"))
            .and(body_json(expected_body))
            .respond_with(ResponseTemplate::new(400).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .auth()
            .refresh()
            .post()
            .build()?
            .send()
            .await
            .expect_err("expected error");

        #[cfg(not(feature = "multi-thread"))]
        assert_eq!(
            mangadex_client.http_client.try_borrow()?.get_tokens(),
            Some(&AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "".to_string(),
            })
        );
        #[cfg(feature = "multi-thread")]
        assert_eq!(
            mangadex_client.http_client.lock().await.get_tokens(),
            Some(&AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "".to_string(),
            })
        );

        if let Error::Api(errors) = res {
            assert_eq!(errors.errors.len(), 1);

            assert_eq!(errors.errors[0].id, error_id);
            assert_eq!(errors.errors[0].status, 400);
            assert_eq!(errors.errors[0].title, Some("Empty token".to_string()));
            assert_eq!(
                errors.errors[0].detail,
                Some("Token cannot be empty".to_string())
            );
        }

        Ok(())
    }

    #[tokio::test]
    async fn refresh_handles_401() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "invalidtoken".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let error_id = Uuid::new_v4();

        let expected_body = json!({"token": "invalidtoken"});
        let response_body = json!({
            "result": "error",
            "errors": [{
                "id": error_id.to_string(),
                "status": 401,
                "title": "Invalid token",
                "detail": "Token is not valid"
            }]
        });

        Mock::given(method("POST"))
            .and(path(r"/auth/refresh"))
            .and(header("Content-Type", "application/json"))
            .and(body_json(expected_body))
            .respond_with(ResponseTemplate::new(401).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .auth()
            .refresh()
            .post()
            .build()?
            .send()
            .await
            .expect_err("expected error");

        #[cfg(not(feature = "multi-thread"))]
        assert_eq!(
            mangadex_client.http_client.try_borrow()?.get_tokens(),
            Some(&AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "invalidtoken".to_string(),
            })
        );
        #[cfg(feature = "multi-thread")]
        assert_eq!(
            mangadex_client.http_client.lock().await.get_tokens(),
            Some(&AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "invalidtoken".to_string(),
            })
        );

        if let Error::Api(errors) = res {
            assert_eq!(errors.errors.len(), 1);

            assert_eq!(errors.errors[0].id, error_id);
            assert_eq!(errors.errors[0].status, 401);
            assert_eq!(errors.errors[0].title, Some("Invalid token".to_string()));
            assert_eq!(
                errors.errors[0].detail,
                Some("Token is not valid".to_string())
            );
        }

        Ok(())
    }

    #[tokio::test]
    async fn refresh_handles_403() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "expiredtoken".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let error_id = Uuid::new_v4();

        let expected_body = json!({"token": "expiredtoken"});
        let response_body = json!({
            "result": "error",
            "errors": [{
                "id": error_id.to_string(),
                "status": 403,
                "title": "Expired token",
                "detail": "Session has expired"
            }]
        });

        Mock::given(method("POST"))
            .and(path(r"/auth/refresh"))
            .and(header("Content-Type", "application/json"))
            .and(body_json(expected_body))
            .respond_with(ResponseTemplate::new(403).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .auth()
            .refresh()
            .post()
            .build()?
            .send()
            .await
            .expect_err("expected error");

        #[cfg(not(feature = "multi-thread"))]
        assert_eq!(
            mangadex_client.http_client.try_borrow()?.get_tokens(),
            Some(&AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "expiredtoken".to_string(),
            })
        );
        #[cfg(feature = "multi-thread")]
        assert_eq!(
            mangadex_client.http_client.lock().await.get_tokens(),
            Some(&AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "expiredtoken".to_string(),
            })
        );

        if let Error::Api(errors) = res {
            assert_eq!(errors.errors.len(), 1);

            assert_eq!(errors.errors[0].id, error_id);
            assert_eq!(errors.errors[0].status, 403);
            assert_eq!(errors.errors[0].title, Some("Expired token".to_string()));
            assert_eq!(
                errors.errors[0].detail,
                Some("Session has expired".to_string())
            );
        }

        Ok(())
    }
}
