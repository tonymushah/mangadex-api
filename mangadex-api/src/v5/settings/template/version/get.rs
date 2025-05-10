//! Builder for getting a Settings template by version ID.
//!
//! <https://api.mangadex.org/docs/swagger.html#/Settings/get-settings-template-version>
//!
//! ```rust
//! use mangadex_api::MangaDexClient;
//! // use mangadex_api_types::{Password, Username};
//! use uuid::Uuid;
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
//! let version_id = Uuid::new_v4();
//!
//! let res = client
//!     .settings()
//!     .template()
//!     .version(version_id)
//!     .get()
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

use crate::HttpClientRef;
use mangadex_api_schema::v5::UserSettingsTemplateResponse;

/// Get a Settings template by version ID.
///
/// This requires authentication.
///
/// Makes a request to `GET /settings/template/{version}`.
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
pub struct GetSettingsTemplateByVersionId {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[allow(unused)]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub http_client: HttpClientRef,

    #[serde(skip_serializing)]
    pub version: Uuid,
}

endpoint! {
    GET ("/settings/template/{}", version),
    #[no_data auth] GetSettingsTemplateByVersionId,
    #[flatten_result] UserSettingsTemplateResponse,
    GetSettingsTemplateByVersionIdBuilder
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use mangadex_api_schema::v5::AuthTokens;
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::error::Error;
    use crate::{HttpClient, MangaDexClient};

    #[tokio::test]
    async fn get_settings_template_by_version_id() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);
        let id = Uuid::from_str("f582e935-effb-4a39-821d-7a512e5b3f55")?;
        let response_body = json!({
            "result": "ok",
            "response": "entity",
            "data": {
                "id": id,
                "type": "settings_template",
                "attributes": {
                    "template": {
                    },
                    "createdAt": "2024-07-29T10:20:22+00:00",
                    "updatedAt": "2024-07-29T10:20:22+00:00",
                    "version": 1
                },
                "relationships": []
            }
        });

        Mock::given(method("GET"))
            .and(path_regex("/settings/template/[0-9a-fA-F-]+"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .settings()
            .template()
            .version(id)
            .get()
            .send()
            .await?;
        assert_eq!(
            res.data.attributes.template,
            serde_json::Value::Object(Default::default())
        );
        Ok(())
    }
    #[tokio::test]
    async fn get_settings_template_by_version_id_requires_auth() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let version_id = Uuid::new_v4();
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

        Mock::given(method("GET"))
            .and(path_regex(r"/settings/template/[0-9a-fA-F-]+"))
            .respond_with(ResponseTemplate::new(403).set_body_json(response_body))
            .expect(0)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .settings()
            .template()
            .version(version_id)
            .get()
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
