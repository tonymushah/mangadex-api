//! Builder for creating a new report.
//!
//! <https://api.mangadex.org/swagger.html#/Report/post-report>
//!
//! # Examples
//!
//! ```rust
//! use uuid::Uuid;
//!
//! use mangadex_api_types::ReportCategory;
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
//! let reason_id = Uuid::new_v4();
//! let manga_id = Uuid::new_v4();
//!
//! let res = client
//!     .report()
//!     .create()
//!     .category(ReportCategory::Manga)
//!     .reason(&reason_id)
//!     .object_id(&manga_id)
//!     .build()?
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
use mangadex_api_types::error::Result;
use mangadex_api_types::ReportCategory;

#[derive(Debug, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), pattern = "owned")]
pub struct CreateReport<'a> {
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    pub(crate) http_client: HttpClientRef,

    pub category: ReportCategory,
    /// The report reason ID for sub-categorization.
    ///
    /// For example, if a manga was being reported for being a troll entry, the specific reason ID should be used, obtained from the [list report reasons endpoint](crate::v5::report::list).
    pub reason: &'a Uuid,
    /// The ID from the category type.
    ///
    /// For example, if the category is "manga", this should be a manga UUID.
    pub object_id: &'a Uuid,
    /// Optional notes about why this is being reported.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub details: Option<&'a str>,
}

endpoint! {
    POST "/report",
    #[body auth] CreateReport<'_>,
    #[discard_result] Result<NoData>
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{header, method, path};
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
        let _expected_body = json!({
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
            // TODO: Make the request body check work.
            // .and(body_json(expected_body))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        mangadex_client
            .report()
            .create()
            .category(ReportCategory::Manga)
            .reason(&reason_id)
            .object_id(&manga_id)
            .build()?
            .send()
            .await?;

        Ok(())
    }
}
