//! Builder for creating a new report.
//!
//! <https://api.mangadex.org/docs/swagger.html#/Report/post-report>
//!
//! # Examples
//!
//! ```rust
//! use uuid::Uuid;
//!
//! use mangadex_api_types::ReportCategory;
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
//!
//! let reason_id = Uuid::new_v4();
//! let manga_id = Uuid::new_v4();
//!
//! let res = client
//!     .report()
//!     .post()
//!     .category(ReportCategory::Manga)
//!     .reason(reason_id)
//!     .object_id(manga_id)
//!     .send()
//!     .await?;
//!
//! println!("report reasons: {:?}", res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;
use mangadex_api_schema::NoData;
use mangadex_api_types::ReportCategory;

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
pub struct CreateReport {
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub http_client: HttpClientRef,

    pub category: ReportCategory,
    /// The report reason ID for sub-categorization.
    ///
    /// For example, if a manga was being reported for being a troll entry, the specific reason ID should be used, obtained from the [list report reasons endpoint](crate::v5::report::get).
    pub reason: Uuid,
    /// The ID from the category type.
    ///
    /// For example, if the category is "manga", this should be a manga UUID.
    pub object_id: Uuid,
    /// Optional notes about why this is being reported.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub details: Option<String>,
}

endpoint! {
    POST "/report",
    #[body auth] CreateReport,
    #[rate_limited] NoData,
    CreateReportBuilder
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{body_json, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::ReportCategory;

    #[tokio::test]
    async fn create_report_reasons_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let reason_id = Uuid::new_v4();
        let manga_id = Uuid::new_v4();
        let expected_body = json!({
            "category": "manga",
            "reason": reason_id,
            "objectId": manga_id,
        });
        let response_body = json!({
            "result": "ok"
        });

        Mock::given(method("POST"))
            .and(path("/report"))
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

        mangadex_client
            .report()
            .post()
            .category(ReportCategory::Manga)
            .reason(reason_id)
            .object_id(manga_id)
            .send()
            .await?;

        Ok(())
    }
}
