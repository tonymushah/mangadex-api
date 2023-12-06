//! Builder for the scanlation group creation endpoint.
//!
//! <https://api.mangadex.org/docs/swagger.html#/ScanlationGroup/post-group>
//!
//! # Examples
//!
//! ```rust
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
//!
//! let res = client
//!     .scanlation_group()
//!     .post()
//!     .name("My Group Name")
//!     .description("This is a short description about this group...")
//!     .send()
//!     .await?;
//!
//! println!("scanlation group create: {:?}", res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;
use url::Url;

use crate::HttpClientRef;
use mangadex_api_schema::v5::GroupData;
use mangadex_api_types::MangaDexDuration;

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
pub struct CreateGroup {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub http_client: HttpClientRef,

    pub name: String,
    /// Nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub website: Option<String>,
    /// Nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub irc_server: Option<String>,
    /// Nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub irc_channel: Option<String>,
    /// Nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub discord: Option<String>,
    /// Nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub contact_email: Option<String>,
    /// Nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub description: Option<String>,
    /// Nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub twitter: Option<Url>,
    /// Regex: [^https:/\/www\.mangaupdates\.com\/(?:groups|publishers)\.html\?id=\d+](https://www.mangaupdates.com)
    ///
    /// Nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub manga_updates: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub inactive: Option<bool>,
    /// Nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub publish_delay: Option<MangaDexDuration>,
}

endpoint! {
    POST ("/group"),
    #[body auth] CreateGroup,
    #[rate_limited] GroupData,
    CreateGroupBuilder
}

#[cfg(test)]
mod tests {
    use fake::faker::lorem::en::Sentence;
    use fake::faker::name::en::Name;
    use fake::Fake;
    use serde_json::json;
    use time::OffsetDateTime;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{body_json, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::MangaDexDateTime;

    #[tokio::test]
    async fn create_scanlation_group_fires_a_request_to_base_url() -> anyhow::Result<()> {
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
        let group_name: String = Name().fake();
        let group_description: String = Sentence(1..2).fake();

        let datetime = MangaDexDateTime::new(&OffsetDateTime::now_utc());

        let expected_body = json!({
            "name": group_name,
            "description": group_description
        });
        let response_body = json!({
            "result": "ok",
            "response": "entity",
            "data": {
                "id": group_id,
                "type": "scanlation_group",
                "attributes": {
                    "name": group_name,
                    "altNames": [],
                    "website": null,
                    "ircServer": null,
                    "ircChannel": null,
                    "discord": null,
                    "contactEmail": null,
                    "description": null,
                    "twitter": null,
                    "focusedLanguages": [],
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
        });

        Mock::given(method("POST"))
            .and(path(r"/group"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .and(header("Content-Type", "application/json"))
            .and(body_json(expected_body))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("x-ratelimit-retry-after", "1698723860")
                    .insert_header("x-ratelimit-limit", "40")
                    .insert_header("x-ratelimit-remaining", "39")
                    .set_body_json(response_body),
            )
            .expect(1)
            .mount(&mock_server)
            .await;

        let _ = mangadex_client
            .scanlation_group()
            .post()
            .name(group_name.as_str())
            .description(group_description.as_str())
            .send()
            .await?;

        Ok(())
    }
}
