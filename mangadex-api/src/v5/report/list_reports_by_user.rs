//! Builder for the report reasons list endpoint.
//!
//! <https://api.mangadex.org/swagger.html#/Report/get-report-reasons-by-category>
//!
//! # Examples
//!
//! ```rust
//! use mangadex_api_types::{ReportCategory, ReportStatus};
//! use mangadex_api::v5::MangaDexClient;
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! let res = client
//!     .report()
//!     .list_reports_by_user()
//!     .category(ReportCategory::Manga)
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("reports: {:?}", res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;

use crate::HttpClientRef;
use mangadex_api_schema::v5::UserReportsListResponse;
use mangadex_api_types::{ReportCategory, ReportSortOrder, ReportStatus, ReferenceExpansionResource};

#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
#[derive(Debug, Serialize, Clone, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default, pattern = "owned", build_fn(error = "mangadex_api_types::error::BuilderError"))]
pub struct ListReportsByUser {
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub(crate) http_client: HttpClientRef,

    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub category: Option<ReportCategory>,
    pub status: Option<ReportStatus>,
    pub order: Option<ReportSortOrder>,
    #[builder(setter(each = "include"))]
    pub includes: Vec<ReferenceExpansionResource>,
}

endpoint! {
    GET "/report",
    #[query auth] ListReportsByUser,
    #[flatten_result] UserReportsListResponse
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use time::OffsetDateTime;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::{MangaDexDateTime, ReportCategory, ReportStatus, ResponseType};

    #[tokio::test]
    async fn list_reports_by_user_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let report_id = Uuid::new_v4();
        let datetime = MangaDexDateTime::new(&OffsetDateTime::now_utc());

        let response_body = json!({
            "result": "ok",
            "response": "collection",
            "data": [
                {
                    "id": report_id,
                    "type": "report",
                    "attributes": {
                        "details": "The manga was a troll submission.",
                        "objectId": "2",
                        "status": "accepted",
                        "createdAt": datetime.to_string()
                    },
                    "relationships": []
                }
            ],
            "limit": 10,
            "offset": 0,
            "total": 1
        });

        Mock::given(method("GET"))
            .and(path("/report"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .report()
            .list_reports_by_user()
            .category(ReportCategory::Manga)
            .build()?
            .send()
            .await?;

        assert_eq!(res.response, ResponseType::Collection);
        assert_eq!(res.limit, 10);
        assert_eq!(res.offset, 0);
        assert_eq!(res.total, 1);
        let report = &res.data[0];
        assert_eq!(report.id, report_id);

        assert_eq!(
            report.attributes.details,
            "The manga was a troll submission.".to_string()
        );
        assert_eq!(report.attributes.object_id, "2".to_string());
        assert_eq!(report.attributes.status, ReportStatus::Accepted);

        Ok(())
    }
}
