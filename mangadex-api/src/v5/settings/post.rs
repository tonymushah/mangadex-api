//! Builder for creating or updating a user's Settings.
//!
//! <https://api.mangadex.org/docs/swagger.html#/Settings/post-settings>
//!
//! ```rust
//! use mangadex_api::MangaDexClient;
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
//!     .settings()
//!     .post()
//!     .send()
//!     .await?;
//!
//! println!("Create Settings: {:?}", res);
//! # Ok(())
//! # }
//! ```

use std::collections::HashMap;

use derive_builder::Builder;
use mangadex_api_schema::v5::UserSettingsAttributes;
// use mangadex_api_schema::v5::UserSettingsResponse;
// use crate::Result;
use mangadex_api_types::MangaDexDateTime;
use serde::Serialize;

use crate::HttpClientRef;

/// Create or update a user's Settings.
///
/// This requires authentication.
///
/// Makes a request to `POST /settings`.
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
pub struct CreateOrUpdateUserSettings {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[allow(unused)]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub http_client: HttpClientRef,

    // TODO: Flesh out body.
    #[builder(default)]
    pub settings: HashMap<String, String>,
    #[builder(default)]
    pub updated_at: MangaDexDateTime,
}

endpoint! {
    POST "/settings",
    #[body auth] CreateOrUpdateUserSettings,
    #[flatten_result] crate::Result<UserSettingsAttributes>,
    CreateOrUpdateUserSettingsBuilder
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::error::Error;
    use crate::{HttpClient, MangaDexClient};

    #[tokio::test]
    async fn create_or_update_user_settings_requires_auth() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let error_id = Uuid::new_v4();
        let response_body = json!({
            "result": "error",
            "errors": [{
                "id": error_id.to_string(),
                "status": 403,
                "title": "Forbidden",
                "detail": "You must be logged in to continue."
            }]
        });

        Mock::given(method("POST"))
            .and(path("/settings"))
            .and(header("Content-Type", "application/json"))
            .respond_with(ResponseTemplate::new(403).set_body_json(response_body))
            .expect(0)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .settings()
            .post()
            .send()
            .await
            .expect_err("expected error");

        match res {
            Error::MissingTokens => {}
            _ => panic!("unexpected error: {:#?}", res),
        }

        Ok(())
    }
}
