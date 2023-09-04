//! Builder for fetching a user's custom lists endpoint.
//!
//! <https://api.mangadex.org/swagger.html#/User/get-user-id-list>
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
//! let user_id = Uuid::new_v4();
//! let res = client
//!     .user()
//!     .custom_lists()
//!     .user_id(&user_id)
//!     .limit(1_u32)
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("custom lists: {:?}", res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;
use mangadex_api_schema::v5::CustomListListResponse;

#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
#[derive(Debug, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), pattern = "owned", build_fn(error = "mangadex_api_types::error::BuilderError"))]
pub struct UserCustomLists {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub(crate) http_client: HttpClientRef,

    #[serde(skip_serializing)]
    pub user_id: Uuid,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    offset: Option<u32>,
}

endpoint! {
    GET ("/user/{}/list", user_id),
    #[query] UserCustomLists,
    #[flatten_result] CustomListListResponse
}

#[cfg(test)]
mod tests {
    use fake::faker::name::en::Name;
    use fake::Fake;
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::{HttpClient, MangaDexClient};

    #[tokio::test]
    async fn get_user_custom_lists_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let user_id = Uuid::new_v4();
        let list_id = Uuid::new_v4();
        let list_name: String = Name().fake();
        let response_body = json!({
            "result": "ok",
            "response": "collection",
            "data": [
                {
                    "id": list_id,
                    "type": "custom_list",
                    "attributes": {
                        "name": list_name,
                        "visibility": "public",
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
            .and(path_regex(r"/user/[0-9a-fA-F-]+/list"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _ = mangadex_client
            .user()
            .custom_lists()
            .user_id(user_id)
            .limit(1_u32)
            .build()?
            .send()
            .await?;

        Ok(())
    }
}
