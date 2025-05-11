//! Builder for creating a Settings template.
//!
//! <https://api.mangadex.org/docs/swagger.html#/Settings/post-settings-template>
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
//!     .template()
//!     .post()
//!     .send()
//!     .await?;
//!
//! println!("Create Settings template: {:?}", res);
//! # Ok(())
//! # }
//! ```

// TODO Implement a schema

use derive_builder::Builder;
use serde::Serialize;

use crate::HttpClientRef;
use crate::Result;
use mangadex_api_schema::NoData;

/// Create a Settings template.
///
/// This requires authentication.
///
/// Makes a request to `POST /settings/template`.
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
pub struct CreateSettingsTemplate {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[allow(unused)]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub http_client: HttpClientRef,

    #[builder(default)]
    #[serde(flatten)]
    pub template: serde_json::Value,
}

endpoint! {
    POST "/settings/template",
    #[body auth] CreateSettingsTemplate,
    #[discard_result] Result<NoData>,
    CreateSettingsTemplateBuilder
}

#[cfg(test)]
mod tests {
    use mangadex_api_schema::v5::AuthTokens;
    use mangadex_api_types::ResultType;
    use serde::Serialize;
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{body_json, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::error::Error;
    use crate::{HttpClient, MangaDexClient};

    #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
    struct TestTemplate {
        description: String,
        value: u32,
    }
    #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
    struct TestTemplateResponse {
        result: ResultType,
        description: String,
        value: u32,
    }
    impl From<TestTemplate> for TestTemplateResponse {
        fn from(value: TestTemplate) -> Self {
            Self {
                result: ResultType::Ok,
                description: value.description,
                value: value.value,
            }
        }
    }
    #[tokio::test]
    async fn create_settings_template_requires_auth() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);
        let resp = TestTemplate {
            description: "Lorem ipsum".into(),
            value: 13,
        };

        let response_body = serde_json::to_value(&resp)?;

        Mock::given(method("POST"))
            .and(path("/settings/template"))
            .and(header("Content-Type", "application/json"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .and(body_json(&resp))
            .respond_with(ResponseTemplate::new(200).set_body_json({
                let body: TestTemplateResponse = resp.into();
                body
            }))
            .mount(&mock_server)
            .await;

        mangadex_client
            .settings()
            .template()
            .post()
            .template(response_body)
            .send()
            .await?;

        Ok(())
    }
    #[tokio::test]
    async fn create_settings_template_requires_auth_403() -> anyhow::Result<()> {
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
            .and(path("/settings/template"))
            .and(header("Content-Type", "application/json"))
            .respond_with(ResponseTemplate::new(403).set_body_json(response_body))
            .expect(0)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .settings()
            .template()
            .post()
            .send()
            .await
            .expect_err("expected error");

        match res {
            Error::MissingTokens => {}
            Error::Api(e) => assert_eq!(e.errors[0].status, 403),
            _ => panic!("unexpected error: {:#?}", res),
        }

        Ok(())
    }
}
