//! Builder for getting a Settings template by version ID.
//!
//! <https://api.mangadex.org/swagger.html#/Settings/get-settings-template-version>
//!
//! ```ignore
//! use mangadex_api::MangaDexClient;
//! use mangadex_api::types::{Password, Username};
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
//! let version_id = Uuid::new_v4();
//!
//! let res = client
//!     .settings()
//!     .get_template_by_version_id()
//!     .version(&version_id)
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
use uuid::Uuid;

// use mangadex_api_schema::NoData;
use crate::HttpClientRef;
// use mangadex_api_types::error::Result;

/// Get a Settings template by version ID.
///
/// This requires authentication.
///
/// Makes a request to `GET /settings/template/{version}`.
#[derive(Debug, Builder, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option))]
#[non_exhaustive]
pub struct GetSettingsTemplateByVersionId<'a> {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[allow(unused)]
    pub(crate) http_client: HttpClientRef,

    #[serde(skip)]
    pub version: &'a Uuid,
}

// endpoint! {
//     GET ("/settings/template/{}", version),
//     #[no_data] GetSettingsTemplateByVersionId<'_>,
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
//     async fn get_settings_template_by_version_id_requires_auth() -> anyhow::Result<()> {
//         let mock_server = MockServer::start().await;
//         let http_client: HttpClient = HttpClient::builder()
//             .base_url(Url::parse(&mock_server.uri())?)
//             .build()?;
//         let mangadex_client = MangaDexClient::new_with_http_client(http_client);

//         let version_id = Uuid::new_v4();
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
//             .and(path_regex(r"/settings/template/[0-9a-fA-F-]+"))
//             .respond_with(ResponseTemplate::new(403).set_body_json(response_body))
//             .expect(0)
//             .mount(&mock_server)
//             .await;

//         let res = mangadex_client
//             .settings()
//             .get_template_by_version_id()
//             .version(&version_id)
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
