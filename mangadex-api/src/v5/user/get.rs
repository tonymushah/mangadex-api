//! Builder for the user list endpoint.
//!
//! Authentication is required. This can be done by logging in.
//!
//! <https://api.mangadex.org/docs/swagger.html#/User/get-user>
//!
//! # Examples
//!
//! ```rust
//! use mangadex_api_types::MangaStatus;
//! use mangadex_api::MangaDexClient;
//! // use mangadex_api_types::{Password, Username};
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
//!
//! let users_res = client
//!     .user()
//!     .get()
//!     .username("holo")
//!     .send()
//!     .await?;
//!
//! println!("users: {:?}", users_res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;
use mangadex_api_schema::v5::UserListResponse;
use mangadex_api_types::UserSortOrder;

#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
#[derive(Debug, Serialize, Clone, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(
    setter(into, strip_option),
    default,
    build_fn(error = "mangadex_api_types::error::BuilderError")
)]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
pub struct ListUser {
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub http_client: HttpClientRef,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[builder(setter(each = "add_user_id"))]
    #[serde(rename = "ids")]
    pub user_ids: Vec<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<UserSortOrder>,
}

endpoint! {
    GET "/user",
    #[query auth] ListUser,
    #[flatten_result] UserListResponse,
    ListUserBuilder
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
    use mangadex_api_types::ResponseType;

    #[tokio::test]
    async fn list_user_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let user_id = Uuid::new_v4();
        let response_body = json!({
            "result": "ok",
            "response": "collection",
            "data": [
                {
                    "id": user_id,
                    "type": "user",
                    "attributes": {
                        "username": "myusername",
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
            ],
            "limit": 1,
            "offset": 0,
            "total": 1
        });

        Mock::given(method("GET"))
            .and(path("/user"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client.user().get().limit(1u32).send().await?;

        assert_eq!(res.response, ResponseType::Collection);
        let user = &res.data[0];
        assert_eq!(user.id, user_id);
        assert_eq!(user.attributes.username, "myusername");
        assert_eq!(user.attributes.version, 1);

        Ok(())
    }

    #[tokio::test]
    async fn list_manga_handles_400() -> anyhow::Result<()> {
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
                "status": 400,
                "title": "Invalid limit",
                "detail": "Limit must be between 1 and 100"
            }]
        });

        Mock::given(method("GET"))
            .and(path("/user"))
            .respond_with(ResponseTemplate::new(400).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .user()
            .get()
            .limit(0u32)
            .send()
            .await
            .expect_err("expected error");

        if let Error::Api(errors) = res {
            assert_eq!(errors.errors.len(), 1);

            assert_eq!(errors.errors[0].id, error_id);
            assert_eq!(errors.errors[0].status, 400);
            assert_eq!(errors.errors[0].title, Some("Invalid limit".to_string()));
            assert_eq!(
                errors.errors[0].detail,
                Some("Limit must be between 1 and 100".to_string())
            );
        }

        Ok(())
    }
}
