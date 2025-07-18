//! Builder for the create client endpoint.
//!
//! <https://api.mangadex.org/docs/swagger.html#/ApiClient/post-create-apiclient>
//! <https://api.mangadex.org/docs/redoc.html#tag/ApiClient/operation/post-create-apiclient>
//!
//! ```rust
//!
//! use mangadex_api::MangaDexClient;
//! // use mangadex_api_types::{Password, Username};
//! use mangadex_api_types::ApiClientProfile;
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! /*
//! Put your login script here
//!  
//! let _login_res = client
//!     .auth()
//!     .login()
//!     .username(Username::parse("myusername")?)
//!     .password(Password::parse("hunter23")?)
//!     .build()?
//!     .send()
//!     .await?;
//! */
//!
//! let client_res = client
//!     .client()
//!     .post()
//!     .name("My Client")
//!     .profile(ApiClientProfile::Personal)
//!     .description("It's my personal API Client for the mangadex-api :)")
//!     .send()
//!     .await?;
//!
//! println!("Client creation: {:?}", client_res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;

use crate::HttpClientRef;
use mangadex_api_types::ApiClientProfile;

type ApiClientResponse = crate::Result<mangadex_api_schema::v5::ApiClientData>;

/// Create a new api client.
///
/// This requires authentication.
///
/// Makes a request to `POST /client`
#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
#[derive(Debug, Serialize, Clone, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(
    setter(into, strip_option),
    build_fn(error = "crate::error::BuilderError")
)]
#[non_exhaustive]
pub struct CreateClient {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub http_client: HttpClientRef,

    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub description: Option<String>,
    #[builder(default)]
    pub profile: ApiClientProfile,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "Some(1)")]
    pub version: Option<u32>,
}

endpoint! {
    POST "/client",
    #[body auth] CreateClient,
    #[flatten_result] ApiClientResponse,
    CreateClientBuilder
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use time::OffsetDateTime;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{body_json, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::{ApiClientProfile, ApiClientState, MangaDexDateTime};
    use serde::Serialize;

    #[derive(Serialize, Clone)]
    struct CreateClientTestBody {
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default)]
        pub profile: ApiClientProfile,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub version: Option<u32>,
    }

    #[tokio::test]
    async fn create_client_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(non_exhaustive::non_exhaustive!(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            }))
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let client_id = Uuid::new_v4();
        let datetime = MangaDexDateTime::new(&OffsetDateTime::now_utc());

        let _expected_body = CreateClientTestBody {
            name: "mangadex-api".to_string(),
            description: Some("a test api for the mangadex-api".to_string()),
            profile: ApiClientProfile::Personal,
            version: Some(1),
        };
        let state = ApiClientState::Requested;
        let response_body = json!({
            "result": "ok",
            "response": "entity",
            "data": {
                "id": client_id,
                "type": "api_client",
                "attributes": {
                    "name": _expected_body.name,
                    "description": _expected_body.description,
                    "profile": _expected_body.profile,
                    "externalClientId": null,
                    "isActive": false,
                    "state": state,
                    "createdAt": datetime.to_string(),
                    "updatedAt": datetime.to_string(),
                    "version": 1
                },
                "relationships": []
            }
        });
        Mock::given(method("POST"))
            .and(path("/client"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .and(header("Content-Type", "application/json"))
            .and(body_json(_expected_body.clone()))
            .respond_with(ResponseTemplate::new(201).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let mut req_binding = mangadex_client.client().post();
        let req = req_binding.name(_expected_body.name.clone());
        if let Some(description) = _expected_body.description.clone() {
            req.description(description);
        }
        req.profile(_expected_body.profile);
        if let Some(version) = _expected_body.version {
            req.version(version);
        }

        let res = req.send().await?;
        let data = res.data;
        assert_eq!(data.id, client_id);
        assert_eq!(data.attributes.name, _expected_body.name);
        assert_eq!(data.attributes.description, _expected_body.description);
        assert!(data.attributes.external_client_id.is_none());
        assert!(!data.attributes.is_active);
        assert_eq!(data.attributes.state, state);
        assert_eq!(data.attributes.created_at.to_string(), datetime.to_string());
        assert_eq!(data.attributes.updated_at.to_string(), datetime.to_string());
        assert_eq!(data.attributes.profile, _expected_body.profile);
        Ok(())
    }
}
