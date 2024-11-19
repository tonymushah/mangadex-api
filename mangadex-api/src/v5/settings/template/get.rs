//! Builder for getting the latest Settings template.
//!
//! <https://api.mangadex.org/docs/swagger.html#/Settings/get-settings-template>
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
//! let res = client
//!     .settings()
//!     .template()
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

use crate::HttpClientRef;
use mangadex_api_schema::v5::UserSettingsTemplateResponse;

/// Get the latest Settings template.
///
/// This requires authentication.
///
/// Makes a request to `GET /settings/template`.
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
pub struct GetLatestSettingsTemplate {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[allow(unused)]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub http_client: HttpClientRef,
}

endpoint! {
    GET "/settings/template",
    #[no_data auth] GetLatestSettingsTemplate,
    #[flatten_result] UserSettingsTemplateResponse,
    GetLatestSettingsTemplateBuilder
}

#[cfg(test)]
mod tests {
    use mangadex_api_schema::v5::AuthTokens;
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::error::Error;

    #[tokio::test]
    async fn get_latest_settings_template() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let response_body = json!({
            "result": "ok",
            "response": "entity",
            "data": {
                "id": "f582e935-effb-4a39-821d-7a512e5b3f55",
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
            .and(path_regex("/settings/template"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client.settings().template().get().send().await?;
        assert_eq!(
            res.data.attributes.template,
            serde_json::Value::Object(Default::default())
        );
        Ok(())
    }
    #[tokio::test]
    async fn get_latest_settings_template_requires_auth() -> anyhow::Result<()> {
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

        Mock::given(method("GET"))
            .and(path_regex("/settings/template"))
            .respond_with(ResponseTemplate::new(403).set_body_json(response_body))
            .expect(0)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .settings()
            .template()
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
