//! Builder for the custom list view endpoint.
//!
//! <https://api.mangadex.org/docs/swagger.html#/CustomList/get-list-id>
//!
//! # Examples
//!
//! ```rust
//! use uuid::Uuid;
//!
//! use mangadex_api::v5::MangaDexClient;
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! let list_id = Uuid::new_v4();
//! let res = client
//!     .custom_list()
//!     .id(list_id)
//!     .get()
//!     .send()
//!     .await?;
//!
//! println!("custom list: {:?}", res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use mangadex_api_schema::v5::CustomListData;
use mangadex_api_types::ReferenceExpansionResource;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;

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
pub struct GetCustomList {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub http_client: HttpClientRef,

    #[serde(skip_serializing)]
    pub list_id: Uuid,

    #[serde(skip_serializing)]
    #[builder(default)]
    pub with_auth: bool,

    #[builder(setter(each = "include"), default)]
    pub includes: Vec<ReferenceExpansionResource>,
}

endpoint! {
    GET ("/list/{}", list_id),
    #[query auth => with_auth] GetCustomList,
    #[flatten_result] crate::Result<CustomListData>,
    GetCustomListBuilder
}

#[cfg(test)]
mod tests {
    use fake::faker::name::en::Name;
    use fake::Fake;
    use mangadex_api_schema::v5::AuthTokens;
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::error::Error;
    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::CustomListVisibility;

    #[tokio::test]
    async fn get_custom_list_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let list_id = Uuid::new_v4();
        let list_name: String = Name().fake();
        let response_body = json!({
            "result": "ok",
            "response": "entity",
            "data": {
                "id": list_id,
                "type": "custom_list",
                "attributes": {
                    "name": list_name,
                    "visibility": "private",
                    "version": 1
                },
                "relationships": []
            }
        });

        Mock::given(method("GET"))
            .and(path_regex(r"/list/[0-9a-fA-F-]+"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .custom_list()
            .id(list_id)
            .get()
            .send()
            .await?;

        assert_eq!(res.data.id, list_id);
        assert_eq!(res.data.attributes.name, list_name);
        assert_eq!(
            res.data.attributes.visibility,
            CustomListVisibility::Private
        );
        assert_eq!(res.data.attributes.version, 1);

        Ok(())
    }

    #[tokio::test]
    async fn get_custom_list_handles_404() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let list_id = Uuid::new_v4();
        let error_id = Uuid::new_v4();

        let response_body = json!({
            "result": "error",
            "errors": [{
                "id": error_id.to_string(),
                "status": 404,
                "title": "Not found",
                "detail": "CustomList could not be found"
            }]
        });

        Mock::given(method("GET"))
            .and(path_regex(r"/list/[0-9a-fA-F-]+"))
            .respond_with(ResponseTemplate::new(404).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .custom_list()
            .id(list_id)
            .get()
            .send()
            .await
            .expect_err("expected error");

        if let Error::Api(errors) = res {
            assert_eq!(errors.errors.len(), 1);

            assert_eq!(errors.errors[0].id, error_id);
            assert_eq!(errors.errors[0].status, 404);
            assert_eq!(errors.errors[0].title, Some("Not found".to_string()));
            assert_eq!(
                errors.errors[0].detail,
                Some("CustomList could not be found".to_string())
            );
        }

        Ok(())
    }

    #[tokio::test]
    async fn get_custom_list_handles_auth() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(non_exhaustive::non_exhaustive!(AuthTokens {
                session: "sessiontoken".into(),
                refresh: "refresh".into(),
            }))
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let list_id = Uuid::new_v4();
        let error_id = Uuid::new_v4();

        let response_body = json!({
            "result": "error",
            "errors": [{
                "id": error_id.to_string(),
                "status": 404,
                "title": "Not found",
                "detail": "CustomList could not be found"
            }]
        });

        Mock::given(method("GET"))
            .and(path_regex(r"/list/[0-9a-fA-F-]+"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .respond_with(ResponseTemplate::new(404).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .custom_list()
            .id(list_id)
            .get()
            .with_auth(true)
            .send()
            .await
            .expect_err("expected error");

        if let Error::Api(errors) = res {
            assert_eq!(errors.errors.len(), 1);

            assert_eq!(errors.errors[0].id, error_id);
            assert_eq!(errors.errors[0].status, 404);
            assert_eq!(errors.errors[0].title, Some("Not found".to_string()));
            assert_eq!(
                errors.errors[0].detail,
                Some("CustomList could not be found".to_string())
            );
        }

        Ok(())
    }
}
