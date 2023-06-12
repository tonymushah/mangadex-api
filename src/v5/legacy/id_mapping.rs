//! Builder for the legacy ID mapping endpoint.
//!
//! <https://api.mangadex.org/swagger.html#/Legacy/post-legacy-mapping>
//!
//! # Examples
//!
//! ```rust
//! use mangadex_api::v5::MangaDexClient;
//! use mangadex_api_types::LegacyMappingType;
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! let id_mappings_res = client
//!     .legacy()
//!     .id_mapping()
//!     .map_type(LegacyMappingType::Manga)
//!     .ids(vec![1, 2])
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("ID mappings: {:?}", id_mappings_res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;

use crate::HttpClientRef;
use mangadex_api_schema::v5::IdMappingListResponse;
use mangadex_api_types::LegacyMappingType;

#[derive(Debug, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), pattern = "owned")]
pub struct LegacyIdMapping {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    pub(crate) http_client: HttpClientRef,

    #[serde(rename = "type")]
    #[builder(setter(name = "map_type"))]
    pub type_: LegacyMappingType,
    #[builder(setter(each = "add_id"))]
    pub ids: Vec<u64>,
}

endpoint! {
    POST "/legacy/mapping",
    #[body] LegacyIdMapping,
    #[flatten_result] IdMappingListResponse
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::error::Error;
    use mangadex_api_types::{LegacyMappingType, ResponseType};

    #[tokio::test]
    async fn legacy_id_mapping_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let mapping_id = Uuid::new_v4();
        let new_id = Uuid::new_v4();
        let _expected_body = json!({
            "type": "manga",
            "ids": [1]
        });
        let response_body = json!({
            "result": "ok",
            "response": "collection",
            "data": [
                {
                    "id": mapping_id,
                    "type": "mapping_id",
                    "attributes": {
                        "type": "manga",
                        "legacyId": 1,
                        "newId": new_id
                    },
                    "relationships": []
                }
            ],
            "limit": 10,
            "offset": 0,
            "total": 1,
        });

        Mock::given(method("POST"))
            .and(path(r"/legacy/mapping"))
            .and(header("Content-Type", "application/json"))
            // TODO: Make the request body check work.
            // .and(body_json(expected_body))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .legacy()
            .id_mapping()
            .map_type(LegacyMappingType::Manga)
            .add_id(1)
            .build()?
            .send()
            .await?;

        assert_eq!(res.response, ResponseType::Collection);
        let manga_map = &res.data[0];
        assert_eq!(manga_map.id, mapping_id);
        assert_eq!(manga_map.attributes.type_, LegacyMappingType::Manga);
        assert_eq!(manga_map.attributes.legacy_id, 1);
        assert_eq!(manga_map.attributes.new_id, new_id);

        Ok(())
    }

    #[tokio::test]
    async fn legacy_id_mapping_handles_400() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let error_id = Uuid::new_v4();

        let _expected_body = json!({
            "type": "group",
            "ids": [0],
        });
        let response_body = json!({
            "result": "error",
            "errors": [{
                "id": error_id.to_string(),
                "status": 400,
                "title": "Invalid Id",
                "detail": "Id cannot be less than 1"
            }]
        });

        Mock::given(method("POST"))
            .and(path(r"/legacy/mapping"))
            .and(header("Content-Type", "application/json"))
            // TODO: Make the request body check work.
            // .and(body_json(expected_body))
            .respond_with(ResponseTemplate::new(400).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .legacy()
            .id_mapping()
            .map_type(LegacyMappingType::Group)
            .add_id(0)
            .build()?
            .send()
            .await
            .expect_err("expected error");

        if let Error::Api(errors) = res {
            assert_eq!(errors.errors.len(), 1);

            assert_eq!(errors.errors[0].id, error_id);
            assert_eq!(errors.errors[0].status, 400);
            assert_eq!(errors.errors[0].title, Some("Invalid Id".to_string()));
            assert_eq!(
                errors.errors[0].detail,
                Some("Id cannot be less than 1".to_string())
            );
        }

        Ok(())
    }
}
