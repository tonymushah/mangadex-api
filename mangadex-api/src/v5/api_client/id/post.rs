//! Builder for editing client by its id.
//!
//! <https://api.mangadex.org/docs/swagger.html#/ApiClient/post-edit-apiclient>
//!
//! # Examples
//!
//! ```rust
//! use mangadex_api::v5::MangaDexClient;
//! use uuid::Uuid;
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! let client_res = client
//!     .client()
//!     .id(Uuid::new_v4())
//!     .post()
//!     .description("My API for testing")
//!     // Normally you don't need this `as u32`
//!     // but for some reason, it need this to compile :(
//!     .version(2 as u32)
//!     .send()
//!     .await?;
//!
//! println!("client: {:?}", client_res);
//! # Ok(())
//! # }
//! ```
//!
use derive_builder::Builder;
use serde::Serialize;

use crate::HttpClientRef;
use uuid::Uuid;

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
pub struct EditClient {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub http_client: HttpClientRef,

    #[serde(skip_serializing)]
    pub client_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub description: Option<String>,
    // >= 1
    pub version: u32,
}

endpoint! {
    POST ("/client/{}", client_id),
    #[body auth] EditClient,
    #[flatten_result] ApiClientResponse,
    EditClientBuilder
}

#[cfg(test)]
mod tests {
    use serde::Serialize;
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{body_json, header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_schema::v5::AuthTokens;
    use mangadex_api_types::RelationshipType;

    #[derive(Serialize, Clone)]
    struct EditClientBody {
        description: Option<String>,
        version: u32,
    }

    #[tokio::test]
    async fn edit_client_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "myToken".to_string(),
                refresh: "myRefreshToken".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let client_id = Uuid::parse_str("eec486de-24f0-4e68-8459-34f26a62ceaa").unwrap();

        let _expected_body: EditClientBody = EditClientBody{
            description: Some("This is a API Client used for the [mangadex-api](https://github.com/tonymushah/mangadex-api) tests.".to_string()),
            version: 1
        };

        let response_body = json!({
          "result": "ok",
          "response": "entity",
          "data": {
            "id": client_id,
            "type": "api_client",
            "attributes": {
              "name": "Mangadex-API-Auth",
              "description": _expected_body.description.clone(),
              "profile": "personal",
              "externalClientId": null,
              "isActive": false,
              "state": "requested",
              "createdAt": "2023-10-28T12:37:22+00:00",
              "updatedAt": "2023-10-28T12:37:22+00:00",
              "version": _expected_body.version
            },
            "relationships": [
              {
                "id": "554149c7-f28f-4a30-b5fa-9db9b1e11353",
                "type": "creator"
              }
            ]
          }
        });

        Mock::given(method("POST"))
            .and(path_regex(r"/client/[0-9a-fA-F-]+"))
            .and(header("Authorization", "Bearer myToken"))
            .and(header("Content-Type", "application/json"))
            .and(body_json(_expected_body.clone()))
            .respond_with(ResponseTemplate::new(201).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .client()
            .id(client_id)
            .post()
            .description(_expected_body.description.clone().unwrap())
            .version(_expected_body.version)
            .send()
            .await?;

        assert_eq!(res.data.type_, RelationshipType::ApiClient);
        assert_eq!(
            res.data.attributes.description,
            _expected_body.description.clone()
        );
        assert_eq!(res.data.attributes.version, _expected_body.version);
        Ok(())
    }
}
