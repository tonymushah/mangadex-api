//! Builder for the client list endpoint.
//!
//! <https://api.mangadex.org/docs/swagger.html#/ApiClient/get-list-apiclients>
//! <https://api.mangadex.org/docs/redoc.html#tag/ApiClient/operation/get-list-apiclients>
//!
//! # Examples
//!
//! ```rust
//! use mangadex_api::v5::MangaDexClient;
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! let client_res = client
//!     .client()
//!     .get()
//!     .send()
//!     .await?;
//!
//! println!("manga: {:?}", client_res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;

use crate::HttpClientRef;
use mangadex_api_schema::v5::ApiClientListResponse;
use mangadex_api_types::{ApiClientState, ReferenceExpansionResource};

// Make a request to `GET /client`
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
pub struct ListClients {
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub(crate) http_client: HttpClientRef,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub state: Option<ApiClientState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub includes: Vec<ReferenceExpansionResource>,
}

endpoint! {
    GET "/client",
    #[query auth] ListClients,
    #[flatten_result] ApiClientListResponse,
    ListClientsBuilder
}

#[cfg(test)]
mod tests {
    use mangadex_api_schema::v5::AuthTokens;
    use serde_json::json;
    use time::OffsetDateTime;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::error::Error;
    use mangadex_api_types::{
        ApiClientProfile, ApiClientState, MangaDexDateTime, RelationshipType, ResponseType,
    };

    #[tokio::test]
    async fn list_client_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "myToken".to_string(),
                refresh: "myRefreshToken".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let client_id = Uuid::new_v4();
        let client_name = "Test Client".to_string();
        let client_description = "A local test client for the Mangadex API".to_string();
        let state = ApiClientState::Requested;
        let datetime = MangaDexDateTime::new(&OffsetDateTime::now_utc());

        let response_body = json!({
            "result": "ok",
            "response": "collection",
            "data": [
                {
                    "id": client_id,
                    "type": "api_client",
                    "attributes": {
                        "name": client_name,
                        "description": client_description,
                        "profile": "personal",
                        "externalClientId": null,
                        "isActive": false,
                        "state": state,
                        "createdAt": datetime.to_string(),
                        "updatedAt": datetime.to_string(),
                        "version": 1
                    },
                    "relationships": []
                }
            ],
            "limit": 1,
            "offset": 0,
            "total": 1
        });

        Mock::given(method("GET"))
            .and(path("/client"))
            .and(header("Authorization", "Bearer myToken"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client.client().get().limit(1u32).send().await?;

        assert_eq!(res.response, ResponseType::Collection);
        let client: &mangadex_api_schema::ApiObject<mangadex_api_schema::v5::ApiClientAttributes> =
            &res.data[0];
        assert_eq!(client.id, client_id);
        assert_eq!(client.type_, RelationshipType::ApiClient);
        assert_eq!(client.attributes.name, client_name);
        assert_eq!(client.attributes.description, Some(client_description));
        assert_eq!(client.attributes.profile, ApiClientProfile::Personal);
        assert_eq!(client.attributes.external_client_id, None);
        assert!(!client.attributes.is_active);
        assert_eq!(client.attributes.state, state);
        assert_eq!(
            client.attributes.created_at.to_string(),
            datetime.to_string()
        );
        assert_eq!(
            client.attributes.updated_at.to_string(),
            datetime.to_string()
        );
        assert_eq!(client.attributes.version, 1);
        Ok(())
    }

    #[tokio::test]
    async fn list_client_handles_400() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "myToken".to_string(),
                refresh: "myRefreshToken".to_string(),
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
            .and(path("/client"))
            .respond_with(ResponseTemplate::new(400).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .client()
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
