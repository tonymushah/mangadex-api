//! Builder for getting the latest Settings template.
//!
//! <https://api.mangadex.org/swagger.html#/Settings/get-settings-template>
//!
//! ```ignore
//! use mangadex_api::MangaDexClient;
//! use mangadex_api_types::{Password, Username};
//! use uuid::Uuid;
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
//!     .get_latest_template()
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("Settings template: {:?}", res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;

// use mangadex_api_schema::NoData;
use crate::HttpClientRef;
// use mangadex_api_types::error::Result;

/// Get the latest Settings template.
///
/// This requires authentication.
///
/// Makes a request to `GET /settings/template`.
#[derive(Debug, Builder, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option))]
#[non_exhaustive]
pub struct GetLatestSettingsTemplate {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[allow(unused)]
    pub(crate) http_client: HttpClientRef,
}

// endpoint! {
//     GET "/settings/template",
//     #[no_data auth] GetLatestSettingsTemplate,
//     #[discard_result] Result<NoData>
// }

// #[cfg(test)]
// mod tests {
//     use serde_json::json;
//     use url::Url;
//     use uuid::Uuid;
//     use wiremock::matchers::{method, path_regex};
//     use wiremock::{Mock, MockServer, ResponseTemplate};

//     use mangadex_api_types::error::Error;
//     use crate::{HttpClient, MangaDexClient};

//     #[tokio::test]
//     async fn get_latest_settings_template_requires_auth() -> anyhow::Result<()> {
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
//             .and(path("/settings/template"))
//             .respond_with(ResponseTemplate::new(403).set_body_json(response_body))
//             .expect(0)
//             .mount(&mock_server)
//             .await;

//         let res = mangadex_client
//             .settings()
//             .get_latest_template()
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
