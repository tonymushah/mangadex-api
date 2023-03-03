//! Builder for the followed scanlation groups endpoint.
//!
//! <https://api.mangadex.org/swagger.html#/Follows/get-user-follows-group>
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
//! let res = client
//!     .user()
//!     .followed_groups()
//!     .limit(1_u32)
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("followed groups: {:?}", res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;

use crate::HttpClientRef;
use mangadex_api_schema::v5::GroupListResponse;
use mangadex_api_types::ReferenceExpansionResource;

#[derive(Debug, Serialize, Clone, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), pattern = "owned", default)]
pub struct FollowedGroups {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    pub(crate) http_client: HttpClientRef,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[builder(setter(each = "include"), default)]
    pub includes: Vec<ReferenceExpansionResource>,
}

endpoint! {
    GET "/user/follows/group",
    #[query auth] FollowedGroups,
    #[flatten_result] GroupListResponse
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use time::OffsetDateTime;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::MangaDexDateTime;

    #[tokio::test]
    async fn get_followed_groups_fires_a_request_to_base_url_ungrouped() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let group_id = Uuid::new_v4();

        let datetime = MangaDexDateTime::new(&OffsetDateTime::now_utc());

        let response_body = json!({
            "result": "ok",
            "response": "collection",
            "data": [
                {
                    "id": group_id,
                    "type": "scanlation_group",
                    "attributes": {
                        "name": "Scanlation Group",
                        "altNames": [],
                        "website": "https://example.org",
                        "ircServer": null,
                        "ircChannel": null,
                        "discord": null,
                        "contactEmail": null,
                        "description": null,
                        "twitter": null,
                        "focusedLanguages": ["en"],
                        "locked": false,
                        "official": false,
                        "verified": false,
                        "inactive": false,
                        "publishDelay": "P6WT5M",
                        "version": 1,
                        "createdAt": datetime.to_string(),
                        "updatedAt": datetime.to_string(),
                    },
                    "relationships": []
                }
            ],
            "limit": 1,
            "offset": 0,
            "total": 1
        });

        Mock::given(method("GET"))
            .and(path_regex(r"/user/follows/group"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _ = mangadex_client
            .user()
            .followed_groups()
            .limit(1_u32)
            .build()?
            .send()
            .await?;

        Ok(())
    }
}
