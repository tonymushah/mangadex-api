//! Builder for fetching the logged-in user's default custom list.
//!
//! NOTICE: This endpoint is not deployed yet on [Mangadex](https://mangadex.org)
//! We'll notice you when it's deployed
//!
//! # Examples
//!
//! ```rust
//! use uuid::Uuid;
//!
//! use mangadex_api::v5::MangaDexClient;
//! // use mangadex_api_types::{Password, Username};
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! /*
//!
//!     let _login_res = client
//!         .auth()
//!         .login()
//!         .post()
//!         .username(Username::parse("myusername")?)
//!         .password(Password::parse("hunter23")?)
//!         .send()
//!         .await?;
//!
//!  */
//!
//! let res = client
//!     .user()
//!     .list()
//!     .default()
//!     .get()
//!     .send()
//!     .await?;
//!
//! println!("custom lists: {:?}", res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use mangadex_api_schema::v5::CustomListCollection;
use mangadex_api_types::ReferenceExpansionResource;
use serde::Serialize;

use crate::HttpClientRef;

#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
#[derive(Debug, Serialize, Clone, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(
    setter(into, strip_option),
    default,
    build_fn(error = "crate::error::BuilderError")
)]
#[non_exhaustive]
pub struct MyDefaultCustomLists {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub http_client: HttpClientRef,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(setter(each = "include"))]
    pub includes: Vec<ReferenceExpansionResource>,
}

endpoint! {
    GET "/user/list/default",
    #[query auth] MyDefaultCustomLists,
    #[flatten_result] crate::Result<CustomListCollection>,
    MyDefaultCustomListsBuilder
}

#[cfg(test)]
mod tests {
    use fake::faker::name::en::Name;
    use fake::Fake;
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};

    #[tokio::test]
    async fn get_my_default_custom_lists_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(non_exhaustive::non_exhaustive!(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            }))
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

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
                        "visibility": "private",
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
            .and(path(r"/user/list/default"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _ = mangadex_client.user().list().default().get().send().await?;

        Ok(())
    }
}
