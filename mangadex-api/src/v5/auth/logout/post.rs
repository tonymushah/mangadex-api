//! Builder for the auth logout endpoint.
//!
//! <https://api.mangadex.org/docs/swagger.html#/Auth/post-auth-logout>
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
//!     .post()
//!     .username(Username::parse("myusername")?)
//!     .password(Password::parse("hunter23")?)
//!     .send()
//!     .await?;
//!
//! client
//!     .auth()
//!     .logout()
//!     .post()
//!     .send()
//!     .await?;
//!
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;

use crate::HttpClientRef;
use mangadex_api_schema::NoData;
use mangadex_api_types::error::Result;

/// Logout of an account.
///
/// Makes a request to `POST /auth/logout`.
// It doesn't make much sense to make this a builder pattern but for consistency, it is.
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
pub struct Logout {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub http_client: HttpClientRef,
}

impl Logout {
    pub async fn send(&self) -> Result<()> {
        #[cfg(all(
            not(feature = "multi-thread"),
            not(feature = "tokio-multi-thread"),
            not(feature = "rw-multi-thread")
        ))]
        {
            self.http_client.try_borrow()?.send_request(self).await??;

            self.http_client.try_borrow_mut()?.clear_auth_tokens();
        }
        #[cfg(any(feature = "multi-thread", feature = "tokio-multi-thread"))]
        {
            self.http_client.lock().await.send_request(self).await??;

            self.http_client.lock().await.clear_auth_tokens();
        }
        #[cfg(feature = "rw-multi-thread")]
        {
            self.http_client.read().await.send_request(self).await??;

            self.http_client.write().await.clear_auth_tokens();
        }

        Ok(())
    }
}

endpoint! {
    POST "/auth/logout",
    #[no_data auth] Logout,
    #[no_send] Result<NoData>
}

builder_send! {
    #[builder] LogoutBuilder,
    #[discard_result] Result<NoData>
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::error::Error;

    #[tokio::test]
    async fn logout_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let response_body = json!({"result": "ok"});

        Mock::given(method("POST"))
            .and(path(r"/auth/logout"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        mangadex_client.auth().logout().post().send().await?;

        #[cfg(all(
            not(feature = "multi-thread"),
            not(feature = "tokio-multi-thread"),
            not(feature = "rw-multi-thread")
        ))]
        assert_eq!(mangadex_client.http_client.borrow().get_tokens(), None);
        #[cfg(any(feature = "multi-thread", feature = "tokio-multi-thread"))]
        assert_eq!(mangadex_client.http_client.lock().await.get_tokens(), None);

        Ok(())
    }

    #[tokio::test]
    async fn logout_handles_503() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let error_id = Uuid::new_v4();

        let response_body = json!({
            "result": "error",
            "errors": [{
                "id": error_id.to_string(),
                "status": 503,
                "title": "The service is unavailable",
                "detail": "Servers are burning"
            }]
        });

        Mock::given(method("POST"))
            .and(path(r"/auth/logout"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .respond_with(ResponseTemplate::new(503).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .auth()
            .logout()
            .post()
            .send()
            .await
            .expect_err("expected error");

        // The auth tokens should still be part of the client because the logout failed.
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

        if let Error::Api(errors) = res {
            assert_eq!(errors.errors.len(), 1);

            assert_eq!(errors.errors[0].id, error_id);
            assert_eq!(errors.errors[0].status, 503);
            assert_eq!(
                errors.errors[0].title,
                Some("The service is unavailable".to_string())
            );
            assert_eq!(
                errors.errors[0].detail,
                Some("Servers are burning".to_string())
            );
        }

        Ok(())
    }
}
