//! Builder for getting client secret by its id.
//!
//! <https://api.mangadex.org/docs/redoc.html#tag/ApiClient/operation/get-apiclient-secret>
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
//!     .secret()
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
use mangadex_api_schema::v5::ApiClientSecret;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;

type ApiClientSecretResponse = crate::Result<ApiClientSecret>;

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
#[non_exhaustive]
pub struct GetClientSecret {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub http_client: HttpClientRef,

    #[serde(skip_serializing)]
    pub client_id: Uuid,
}

endpoint! {
    GET ("/client/{}/secret", client_id),
    #[query auth] GetClientSecret,
    #[flatten_result] ApiClientSecretResponse,
    GetClientSecretBuilder
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    use non_exhaustive::non_exhaustive;
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_schema::v5::AuthTokens;
    #[tokio::test]
    async fn get_client_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(non_exhaustive!(AuthTokens {
                session: "myToken".to_string(),
                refresh: "myRefreshToken".to_string()
            }))
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let client_id = Uuid::new_v4();
        let data = {
            let mut default_hasher = DefaultHasher::new();
            client_id.hash(&mut default_hasher);
            default_hasher.finish()
        }
        .to_string();
        let response_body = json!({
            "result": "ok",
            "data": data
        });

        Mock::given(method("GET"))
            .and(path_regex(r"/client/[0-9a-fA-F-]+/secret"))
            .and(header("Authorization", "Bearer myToken"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .client()
            .id(client_id)
            .secret()
            .get()
            .send()
            .await?;

        assert_eq!(res.data, data.clone());
        Ok(())
    }
}
