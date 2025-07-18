//! Builder for the unfollow CustomList endpoint.
//!
//! NOTICE : This endpoint is not deploy yet on <https://mangadex.org>
//! We'll notice you if it will be deployed
//!
//! Please use [`mangadex_api::v5::custom_list::id::follow::delete::UnFollowCustomList`] instead
//!
//! <https://api.mangadex.org/swagger.html#/CustomList/unfollow-list-id>
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
//!     let _login_res = client
//!         .auth()
//!         .login()
//!         .username(Username::parse("myusername")?)
//!         .password(Password::parse("hunter23")?)
//!         .send()
//!         .await?;
//! */
//!
//! let list_id = Uuid::new_v4();
//! let _ = client
//!     .custom_list()
//!     .id(list_id)
//!     .bookmark()
//!     .delete()
//!     .send()
//!     .await?;
//!
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use mangadex_api_schema::NoData;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;
use crate::Result;

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
pub struct UnBookMarkCustomList {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub http_client: HttpClientRef,

    /// CustomList ID.
    #[serde(skip_serializing)]
    pub list_id: Uuid,
}

endpoint! {
    DELETE ("/list/{}/bookmark", list_id),
    #[body auth] UnBookMarkCustomList,
    #[discard_result] Result<NoData>,
    UnBookMarkCustomListBuilder
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};

    #[tokio::test]
    async fn unbookmark_custom_list_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(non_exhaustive::non_exhaustive!(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            }))
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let custom_list_id = Uuid::new_v4();
        let response_body = json!({
            "result": "ok"
        });

        Mock::given(method("DELETE"))
            .and(path_regex(r"/list/[0-9a-fA-F-]+/bookmark"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        mangadex_client
            .custom_list()
            .id(custom_list_id)
            .bookmark()
            .delete()
            .send()
            .await?;

        Ok(())
    }
}
