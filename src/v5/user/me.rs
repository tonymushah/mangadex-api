//! Builder for the logged-in user details endpoint.
//!
//! <https://api.mangadex.org/swagger.html#/User/get-user-me>
//!
//! # Examples
//!
//! ```rust
//! use mangadex_api::MangaDexClient;
//! use mangadex_api::types::{Password, Username};
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
//!     .me()
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("me: {:?}", res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;

use crate::HttpClientRef;
use mangadex_api_schema::v5::UserResponse;

#[derive(Debug, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), pattern = "owned")]
pub struct GetMyUserDetails {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    pub(crate) http_client: HttpClientRef,
}

endpoint! {
    GET "/user/me",
    #[query] GetMyUserDetails,
    #[flatten_result] UserResponse
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::{HttpClient, MangaDexClient};

    #[tokio::test]
    async fn get_my_details_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let user_id = Uuid::new_v4();
        let response_body = json!({
            "result": "ok",
            "response": "entity",
            "data": {
                "id": user_id,
                "type": "user",
                "attributes": {
                    "username": "myusername",
                    "roles": [
                        "ROLE_MEMBER",
                        "ROLE_GROUP_MEMBER",
                        "ROLE_GROUP_LEADER"
                    ],
                    "version": 1,
                },
                "relationships": [
                    {
                        "id": "a3219a4f-73c0-4213-8730-05985130539a",
                        "type": "scanlation_group"
                    }
                ]
            }
        });

        Mock::given(method("GET"))
            .and(path(r"/user/me"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _ = mangadex_client.user().me().build()?.send().await?;

        Ok(())
    }
}
