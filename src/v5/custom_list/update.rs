//! Builder for the CustomList update endpoint.
//!
//! <https://api.mangadex.org/swagger.html#/CustomList/put-list-id>
//!
//! # Examples
//!
//! ```rust
//! use uuid::Uuid;
//!
//! use mangadex_api::v5::MangaDexClient;
//! use mangadex_api::types::{Password, Username};
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! let _login_res = client
//!     .auth()
//!     .login()
//!     .username(Username::parse("myusername")?)
//!     .password(Password::parse("hunter23")?)
//!     .build()?
//!     .send()
//!     .await?;
//!
//! let list_id = Uuid::new_v4();
//! let res = client
//!     .custom_list()
//!     .update()
//!     .list_id(&list_id)
//!     .name("Updated List Name")
//!     .version(2u32)
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("update: {:?}", res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;
use mangadex_api_schema::v5::CustomListResponse;
use mangadex_api_types::CustomListVisibility;

#[derive(Debug, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), pattern = "owned")]
#[non_exhaustive]
pub struct UpdateCustomList<'a> {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    pub(crate) http_client: HttpClientRef,

    #[serde(skip)]
    pub list_id: &'a Uuid,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub name: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub visibility: Option<CustomListVisibility>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(setter(each = "add_manga_id"), default)]
    pub manga: Vec<Uuid>,

    pub version: u32,
}

endpoint! {
    PUT ("/list/{}", list_id),
    #[body auth] UpdateCustomList<'_>,
    #[flatten_result] CustomListResponse
}

#[cfg(test)]
mod tests {
    use fake::faker::name::en::Name;
    use fake::Fake;
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{body_json, header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};

    #[tokio::test]
    async fn update_custom_list_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let list_id = Uuid::new_v4();
        let list_name: String = Name().fake();
        let expected_body = json!({
            "version": 2
        });
        let response_body = json!({
            "result": "ok",
            "response": "entity",
            "data": {
                "id": list_id,
                "type": "custom_list",
                "attributes": {
                    "name": list_name,
                    "visibility": "private",
                    "version": 2
                },
                "relationships": []
            }
        });

        Mock::given(method("PUT"))
            .and(path_regex(r"/list/[0-9a-fA-F-]+"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .and(header("Content-Type", "application/json"))
            .and(body_json(expected_body))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _ = mangadex_client
            .custom_list()
            .update()
            .list_id(&list_id)
            .version(2u32)
            .build()?
            .send()
            .await?;

        Ok(())
    }
}
