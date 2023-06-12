//! Builder for getting a user's Settings.
//!
//! <https://api.mangadex.org/swagger.html#/Settings/get-settings>
//!
//! ```ignore
//! use mangadex_api::MangaDexClient;
//! use mangadex_api_types::{Password, Username};
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
//!     .settings()
//!     .get_user_settings()
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("User Settings: {:?}", res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;

// use mangadex_api_schema::v5::UserSettingsResponse;
use crate::HttpClientRef;
// use mangadex_api_types::error::Result;

/// Getting a user's Settings.
///
/// This requires authentication.
///
/// Makes a request to `GET /settings`.
#[derive(Debug, Builder, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option))]
#[non_exhaustive]
pub struct GetUserSettings {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[allow(unused)]
    pub(crate) http_client: HttpClientRef,
}

// endpoint! {
//     GET "/settings",
//     #[no_data auth] GetUserSettings,
//     #[flatten_result] UserSettingsResponse
// }

// #[cfg(test)]
// mod tests {
//     use serde_json::json;
//     use url::Url;
//     use uuid::Uuid;
//     use wiremock::matchers::{method, path};
//     use wiremock::{Mock, MockServer, ResponseTemplate};

//     use mangadex_api_types::error::Error;
//     use crate::{HttpClient, MangaDexClient};

//     #[tokio::test]
//     async fn get_user_settings_requires_auth() -> anyhow::Result<()> {
//         let mock_server = MockServer::start().await;
//         let http_client: HttpClient = HttpClient::builder()
//             .base_url(Url::parse(&mock_server.uri())?)
//             .build()?;
//         let mangadex_client = MangaDexClient::new_with_http_client(http_client);

//         let error_id = Uuid::new_v4();
//         let response_body = json!({
//             "result": "error",
//             "errors": [{
//                 "id": error_id.to_string(),
//                 "status": 403,
//                 "title": "Forbidden",
//                 "detail": "You must be logged in to continue."
//             }]
//         });

//         Mock::given(method("GET"))
//             .and(path("/settings"))
//             .respond_with(ResponseTemplate::new(403).set_body_json(response_body))
//             .expect(0)
//             .mount(&mock_server)
//             .await;

//         let res = mangadex_client
//             .settings()
//             .get_user_settings()
//             .build()?
//             .send()
//             .await
//             .expect_err("expected error");

//         match res {
//             Error::MissingTokens => {}
//             _ => panic!("unexpected error: {:#?}", res),
//         }

//         Ok(())
//     }
// }
