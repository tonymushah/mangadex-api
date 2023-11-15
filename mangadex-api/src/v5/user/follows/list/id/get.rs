//! Builder for checking if the logged-in user follows a custom list.
//!
//! <https://api.mangadex.org/swagger.html#/Follows/get-user-follows-list-id>
//!
//! # Examples
//!
//! ```rust
//! use uuid::Uuid;
//!
//! // use mangadex_api_types::{Password, Username};
//! use mangadex_api::MangaDexClient;
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! /*
//!
//!     let _login_res = client
//!         .auth()
//!         .login()
//!         .post()
//!         .username(Username::parse("myusername")?)
//!         .password(Password::parse("hunter23")?)
//!         .send()
//!         .await?;
//!
//!  */
//!
//! let custom_list_id = Uuid::new_v4();
//!
//! let res = client
//!     .user()
//!     .follows()
//!     .list()
//!     .id(custom_list_id)
//!     .get()
//!     .send()
//!     .await?;
//!
//! println!("check: {:?}", res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use mangadex_api_schema::v5::IsFollowingResponse;
use mangadex_api_schema::{FromResponse, NoData};
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;
use mangadex_api_types::error::{Error, Result};

/// Check if the logged-in user follows a custom list.
///
/// Makes a request to `GET /user/follows/list/{id}`.
#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
#[derive(Debug, Builder, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[builder(
    setter(into, strip_option),
    build_fn(error = "mangadex_api_types::error::BuilderError")
)]
#[deprecated(
    since = "3.0.0-alpha.1",
    note = "After the introduction of the Subscription system, this endpoint will be removed in 3.0.0"
)]
pub struct IsFollowingCustomList {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub http_client: HttpClientRef,

    pub list_id: Uuid,
}

impl IsFollowingCustomList {
    pub async fn send(&mut self) -> Result<IsFollowingResponse> {
        #[cfg(all(
            not(feature = "multi-thread"),
            not(feature = "tokio-multi-thread"),
            not(feature = "rw-multi-thread")
        ))]
        let res = self
            .http_client
            .try_borrow()?
            .send_request_without_deserializing(self)
            .await?;
        #[cfg(any(feature = "multi-thread", feature = "tokio-multi-thread"))]
        let res = self
            .http_client
            .lock()
            .await
            .send_request_without_deserializing(self)
            .await?;
        #[cfg(feature = "rw-multi-thread")]
        let res = self
            .http_client
            .read()
            .await
            .send_request_without_deserializing(self)
            .await?;

        match res.status() {
            reqwest::StatusCode::OK => Ok(IsFollowingResponse { is_following: true }),
            reqwest::StatusCode::NOT_FOUND => {
                let result = res
                    .json::<<Result<NoData> as FromResponse>::Response>()
                    .await?;
                match result.into_result() {
                    Ok(_) => Ok(IsFollowingResponse {
                        is_following: false,
                    }),
                    Err(err) => Err(Error::Api(err)),
                }
            }
            other_status => Err(Error::ServerError(other_status.as_u16(), res.text().await?)),
        }
    }
}

endpoint! {
    GET ("/user/follows/list/{}", list_id),
    #[no_data auth] IsFollowingCustomList,
    #[no_send] Result<IsFollowingResponse>
}

builder_send! {
    #[builder] IsFollowingCustomListBuilder,
    IsFollowingResponse
}

#[cfg(test)]
mod tests {
    use mangadex_api_types::error::Error;
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};

    #[tokio::test]
    async fn user_follows_custom_list() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let list_id = Uuid::new_v4();
        let response_body = json!({
            "result": "ok"
        });

        Mock::given(method("GET"))
            .and(path_regex(r"/user/follows/list/[0-9a-fA-F-]+"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .user()
            .follows()
            .list()
            .id(list_id)
            .get()
            .send()
            .await?;

        assert!(res.is_following);

        Ok(())
    }

    #[tokio::test]
    async fn user_does_not_follow_custom_list() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let list_id = Uuid::new_v4();
        let response_body = json!({
            "result": "ok"
        });

        Mock::given(method("GET"))
            .and(path_regex(r"/user/follows/list/[0-9a-fA-F-]+"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .respond_with(ResponseTemplate::new(404).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .user()
            .follows()
            .list()
            .id(list_id)
            .get()
            .send()
            .await?;

        assert!(!res.is_following);

        Ok(())
    }

    #[tokio::test]
    async fn custom_list_does_not_exist() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let list_id = Uuid::new_v4();
        let error_id = Uuid::new_v4();
        let response_body = json!({
            "result": "error",
            "errors": [{
                "id": error_id.to_string(),
                "status": 404,
                "title": "Custom list does not exist",
                "detail": "The provided custom list does not exist or has been deleted"
            }]
        });

        Mock::given(method("GET"))
            .and(path_regex(r"/user/follows/list/[0-9a-fA-F-]+"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .respond_with(ResponseTemplate::new(404).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .user()
            .follows()
            .list()
            .id(list_id)
            .get()
            .send()
            .await
            .unwrap_err();

        match res {
            Error::Api(error_res) => {
                assert_eq!(error_res.errors.len(), 1);
                assert_eq!(error_res.errors[0].status, 404);
                assert_eq!(
                    error_res.errors[0].title.as_ref().unwrap(),
                    "Custom list does not exist"
                );
            }
            _ => panic!("did not get Error::Api"),
        }

        Ok(())
    }
}
