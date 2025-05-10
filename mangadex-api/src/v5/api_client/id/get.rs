//! Builder for getting client by its id.
//!
//! <https://api.mangadex.org/docs/swagger.html#/ApiClient/get-apiclient>
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
//!     .get()
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
use uuid::Uuid;

use crate::HttpClientRef;
use mangadex_api_types::ReferenceExpansionResource;

type ApiClientResponse = crate::Result<mangadex_api_schema::v5::ApiClientObject>;

#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
#[derive(Debug, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(
    setter(into, strip_option),
    build_fn(error = "crate::error::BuilderError")
)]
pub struct GetClient {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub http_client: HttpClientRef,

    #[serde(skip_serializing)]
    pub client_id: Uuid,

    #[builder(setter(each = "include"), default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub includes: Vec<ReferenceExpansionResource>,
}

endpoint! {
    GET ("/client/{}", client_id),
    #[query auth] GetClient,
    #[flatten_result] ApiClientResponse,
    GetClientBuilder
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_schema::v5::{AuthTokens, RelatedAttributes};
    use mangadex_api_types::{ReferenceExpansionResource, RelationshipType};

    #[tokio::test]
    async fn get_client_fires_a_request_to_base_url() -> anyhow::Result<()> {
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

        let response_body = json!({
          "result": "ok",
          "response": "entity",
          "data": {
            "id": client_id,
            "type": "api_client",
            "attributes": {
              "name": "Mangadex-API-Auth",
              "description": "This is a API Client used for the [mangadex-api](https://github.com/tonymushah/mangadex-api) tests.",
              "profile": "personal",
              "externalClientId": null,
              "isActive": false,
              "state": "requested",
              "createdAt": "2023-10-28T12:37:22+00:00",
              "updatedAt": "2023-10-28T12:37:22+00:00",
              "version": 1
            },
            "relationships": [
              {
                "id": "554149c7-f28f-4a30-b5fa-9db9b1e11353",
                "type": "creator"
              }
            ]
          }
        });

        Mock::given(method("GET"))
            .and(path_regex(r"/client/[0-9a-fA-F-]+"))
            .and(header("Authorization", "Bearer myToken"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client.client().id(client_id).get().send().await?;

        assert_eq!(res.data.type_, RelationshipType::ApiClient);

        Ok(())
    }

    #[tokio::test]
    async fn get_client_handles_reference_expansion() -> anyhow::Result<()> {
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

        let response_body = json!({
          "result": "ok",
          "response": "entity",
          "data": {
            "id": client_id,
            "type": "api_client",
            "attributes": {
              "name": "Mangadex-API-Auth",
              "description": "This is a API Client used for the [mangadex-api](https://github.com/tonymushah/mangadex-api) tests.",
              "profile": "personal",
              "externalClientId": null,
              "isActive": false,
              "state": "requested",
              "createdAt": "2023-10-28T12:37:22+00:00",
              "updatedAt": "2023-10-28T12:37:22+00:00",
              "version": 1
            },
            "relationships": [
              {
                "id": "554149c7-f28f-4a30-b5fa-9db9b1e11353",
                "type": "creator",
                "attributes": {
                  "username": "Tony_Mushah",
                  "roles": [
                    "ROLE_USER"
                  ],
                  "version": 175
                }
              }
            ]
          }
        });

        Mock::given(method("GET"))
            .and(path_regex(r"/client/[0-9a-fA-F-]+"))
            .and(header("Authorization", "Bearer myToken"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .client()
            .id(client_id)
            .get()
            .include(&ReferenceExpansionResource::Creator)
            .send()
            .await?;

        assert_eq!(res.data.relationships[0].type_, RelationshipType::Creator);
        assert!(res.data.relationships[0].related.is_none());
        if let RelatedAttributes::User(user) =
            res.data.relationships[0].attributes.as_ref().unwrap()
        {
            assert_eq!(user.username, "Tony_Mushah".to_string());
        } else {
            panic!("Expected user RelatedAttributes");
        }

        Ok(())
    }
}
