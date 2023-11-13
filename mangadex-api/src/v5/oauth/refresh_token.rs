//! Builder for the OAuth login endpoint.
//!
//! <https://api.mangadex.org/docs/02-authentication/>
//!
//! It's the support for [Personal Client](https://api.mangadex.org/docs/02-authentication/personal-clients/)
//!
//! # Examples
//!
//! ```rust
//! use mangadex_api_types::{Password, Username};
//! use mangadex_api::v5::MangaDexClient;
//! use mangadex_api_schema::v5::oauth::ClientInfo;
//!
//! # async fn run() -> anyhow::Result<()> {
//!
//! let mut client = MangaDexClient::default();
//!
//! client.set_client_info(&ClientInfo {
//!     client_id: "someClientId".to_string(),
//!     client_secret: "someClientSecret".to_string()
//! }).await?;
//!
//! // Login to your account
//!
//! let login_res = client
//!     .oauth()
//!     .login()
//!     .username(Username::parse("myusername")?)
//!     .password(Password::parse("hunter2")?)
//!     .send()
//!     .await?;
//!
//! println!("login: {:?}", login_res);
//!
//! // Wait until the access token expires
//! tokio::time::sleep(tokio::time::Duration::from_secs(<usize as TryInto<u64>>::try_into(login_res.expires_in)?)).await;
//!
//! let refresh_res = client
//!     .oauth()
//!     .refresh()
//!     .send()
//!     .await?;
//!
//! println!("refresh: {:?}", refresh_res);
//!
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use mangadex_api_schema::v5::oauth::OAuthTokenResponse;
use mangadex_api_schema::v5::AuthTokens;
use mangadex_api_types::oauth::GrantTypeSupported;
use reqwest::Method;
use serde::Serialize;
#[cfg(not(test))]
use url::Url;

use crate::v5::HttpClientRef;
use mangadex_api_types::error::Result;

/// Log into an account.
///
/// Makes a request to `POST https://auth.mangadex.org/realms/mangadex/protocol/openid-connect/token`.
#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
#[derive(Debug, Clone, Builder)]
#[builder(
    setter(into, strip_option),
    build_fn(error = "mangadex_api_types::error::BuilderError")
)]
pub struct RefreshTokens {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[cfg_attr(feature = "deserializable-endpoint", serde(skip))]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub http_client: HttpClientRef,
}

#[derive(Clone, Serialize)]
struct RefreshTokenBody {
    grant_type: GrantTypeSupported,
    refresh_token: String,
    client_id: String,
    client_secret: String,
}

impl RefreshTokens {
    pub async fn send(&mut self) -> Result<OAuthTokenResponse> {
        let res = {
            let client = {
                #[cfg(all(
                    not(feature = "multi-thread"),
                    not(feature = "tokio-multi-thread"),
                    not(feature = "rw-multi-thread")
                ))]
                {
                    &self.http_client.try_borrow()?
                }
                #[cfg(any(feature = "multi-thread", feature = "tokio-multi-thread"))]
                {
                    &self.http_client.lock().await
                }
                #[cfg(feature = "rw-multi-thread")]
                {
                    &self.http_client.read().await
                }
            };
            let client_info = client
                .get_client_info()
                .ok_or(mangadex_api_types::error::Error::MissingClientInfo)?;
            let auth_tokens = client
                .get_tokens()
                .ok_or(mangadex_api_types::error::Error::MissingTokens)?;
            let params = RefreshTokenBody {
                grant_type: GrantTypeSupported::RefreshToken,
                refresh_token: auth_tokens.refresh.to_owned(),
                client_id: client_info.client_id.to_owned(),
                client_secret: client_info.client_secret.to_owned(),
            };
            #[cfg(test)]
            let res = client
                .client
                .request(
                    Method::POST,
                    client
                        .base_url
                        .join("/realms/mangadex/protocol/openid-connect/token")?,
                )
                .form(&params)
                .send()
                .await?;
            #[cfg(not(test))]
            let res = client
                .client
                .request(
                    Method::POST,
                    Url::parse(crate::AUTH_URL)?
                        .join("/realms/mangadex/protocol/openid-connect/token")?,
                )
                .form(&params)
                .send()
                .await?;
            res.json::<OAuthTokenResponse>().await?
        };
        {
            let auth_tokens: AuthTokens = From::from(res.clone());
            let client = {
                #[cfg(all(
                    not(feature = "multi-thread"),
                    not(feature = "tokio-multi-thread"),
                    not(feature = "rw-multi-thread")
                ))]
                {
                    &mut self.http_client.try_borrow_mut()?
                }
                #[cfg(any(feature = "multi-thread", feature = "tokio-multi-thread"))]
                {
                    &mut self.http_client.lock().await
                }
                #[cfg(feature = "rw-multi-thread")]
                {
                    &mut self.http_client.write().await
                }
            };
            client.set_auth_tokens(&auth_tokens);
        };
        Ok(res)
    }
}

builder_send! {
    #[builder] RefreshTokensBuilder,
    OAuthTokenResponse
}

#[cfg(test)]
mod tests {
    use mangadex_api_schema::v5::oauth::ClientInfo;
    use mangadex_api_types::oauth::GrantTypeSupported;
    use serde_json::json;
    use url::Url;
    use wiremock::matchers::{body_string, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::v5::oauth::refresh_token::RefreshTokenBody;
    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};
    use serde_urlencoded::to_string;

    #[tokio::test]
    async fn refresh_token_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mut mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let client_info: ClientInfo = ClientInfo {
            client_id: "someClientId".to_string(),
            client_secret: "someClientSecret".to_string(),
        };

        mangadex_client.set_client_info(&client_info).await?;

        let auth_tokens = AuthTokens {
            session: "sessiontoken".to_string(),
            refresh: "refreshtoken".to_string(),
        };

        mangadex_client.set_auth_tokens(&auth_tokens).await?;

        let response_body = json!({
            "access_token": auth_tokens.session.clone(),
            "expires_in": 900,
            "refresh_expires_in": 2414162,
            "refresh_token": auth_tokens.refresh.clone(),
            "token_type": "Bearer",
            "not-before-policy": 0,
            "session_state": "c176499d-6e8d-4ddf-ad59-6d922be66431",
            "scope": "groups email profile",
            "client_type": "personal"
        });
        let expected_body: String = to_string(RefreshTokenBody {
            grant_type: GrantTypeSupported::RefreshToken,
            refresh_token: auth_tokens.refresh.to_owned(),
            client_id: client_info.client_id.clone(),
            client_secret: client_info.client_secret.clone(),
        })?;

        Mock::given(method("POST"))
            .and(path(r"/realms/mangadex/protocol/openid-connect/token"))
            .and(header("Content-Type", "application/x-www-form-urlencoded"))
            .and(body_string(expected_body))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _ = mangadex_client.oauth().refresh().send().await?;

        #[cfg(all(
            not(feature = "multi-thread"),
            not(feature = "tokio-multi-thread"),
            not(feature = "rw-multi-thread")
        ))]
        assert_eq!(
            mangadex_client.http_client.try_borrow()?.get_tokens(),
            Some(&auth_tokens)
        );
        #[cfg(any(feature = "multi-thread", feature = "tokio-multi-thread"))]
        assert_eq!(
            mangadex_client.http_client.lock().await.get_tokens(),
            Some(&auth_tokens)
        );
        #[cfg(feature = "rw-multi-thread")]
        assert_eq!(
            mangadex_client.http_client.read().await.get_tokens(),
            Some(&auth_tokens)
        );

        Ok(())
    }
}
