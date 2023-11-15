//! Builder for removing a batch of manga from a custom list.
//!
//! NOTICE : This endpoint is not currenlty deployed yet on mangadex.org
//! We will give more information when it will be deployed
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
//!     .batch_manga()
//!     .delete()
//!     .manga_id(Uuid::new_v4())
//!     .manga_id(Uuid::new_v4())
//!     .manga_id(Uuid::new_v4())
//!     .send()
//!     .await?;
//!
//! println!("custom list: {:?}", res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use mangadex_api_schema::NoData;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;
use mangadex_api_types::error::Result;

#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
#[derive(Debug, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(
    setter(into, strip_option),
    build_fn(error = "mangadex_api_types::error::BuilderError")
)]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
pub struct DeleteMangaBatchViaCustomList {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub http_client: HttpClientRef,

    #[serde(skip_serializing)]
    /// CustomList ID.
    pub list_id: Uuid,

    #[builder(default)]
    #[builder(setter(each = "manga_id"))]
    pub manga_ids: Vec<Uuid>,
}

endpoint! {
    DELETE ("/list/{}/batch-manga", list_id),
    #[body auth] DeleteMangaBatchViaCustomList,
    #[flatten_result] Result<NoData>,
    DeleteMangaBatchViaCustomListBuilder
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{body_json, header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};

    #[tokio::test]
    async fn delete_manga_by_batch_custom_list_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let custom_list_id = Uuid::new_v4();
        let response_body = json!({
            "result": "ok"
        });

        Mock::given(method("DELETE"))
            .and(path_regex(r"/list/[0-9a-fA-F-]+/batch-manga"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        mangadex_client
            .custom_list()
            .id(custom_list_id)
            .batch_manga()
            .delete()
            .send()
            .await?;

        Ok(())
    }
    #[tokio::test]
    async fn delete_manga_by_batch_with_batch_custom_list_fires_a_request_to_base_url(
    ) -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let custom_list_id = Uuid::new_v4();
        let manga_ids = vec![Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()];
        let response_body = json!({
            "result": "ok"
        });

        let endpoint_ = mangadex_client
            .custom_list()
            .id(custom_list_id)
            .batch_manga()
            .delete()
            .manga_ids(manga_ids)
            .build()?;
        println!("{}", serde_json::to_string_pretty(&(endpoint_.clone()))?);
        Mock::given(method("DELETE"))
            .and(path_regex(r"/list/[0-9a-fA-F-]+/batch-manga"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .and(body_json(endpoint_.clone()))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        endpoint_.send().await?;

        Ok(())
    }
}
