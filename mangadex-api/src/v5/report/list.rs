//! Builder for the report reasons list endpoint.
//!
//! <https://api.mangadex.org/swagger.html#/Report/get-report-reasons-by-category>
//!
//! # Examples
//!
//! ```rust
//! use mangadex_api_types::ReportCategory;
//! use mangadex_api::v5::MangaDexClient;
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! // Known issue: Despite the MangaDex API documents stating that authorization is required,
//! // this endpoint is available to guests.
//!
//! let res = client
//!     .report()
//!     .list()
//!     .category(ReportCategory::Manga)
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

use crate::HttpClientRef;
use mangadex_api_schema::v5::ReportReasonListResponse;
use mangadex_api_types::ReportCategory;

#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
#[derive(Debug, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(
    setter(into, strip_option),
    pattern = "owned",
    build_fn(error = "mangadex_api_types::error::BuilderError")
)]
pub struct ListReasons {
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub(crate) http_client: HttpClientRef,

    #[serde(skip_serializing)]
    pub category: ReportCategory,
}

endpoint! {
    GET ("/report/reasons/{}", category),
    // Known issue: Despite the API docs stating that authorization is required, the endpoint is available to guests.
    #[no_data] ListReasons,
    #[flatten_result] ReportReasonListResponse
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::{Language, ReportCategory, ResponseType};

    #[tokio::test]
    async fn list_report_reasons_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let report_id = Uuid::new_v4();
        let response_body = json!({
            "result": "ok",
            "response": "collection",
            "data": [
                {
                    "id": report_id,
                    "type": "report_reason",
                    "attributes": {
                        "reason": {
                            "en": "Troll entry"
                        },
                        "detailsRequired": false,
                        "category": "manga",
                        "version": 1
                    }
                    // No relationships are returned.
                }
            ],
            "limit": 1,
            "offset": 0,
            "total": 1
        });

        Mock::given(method("GET"))
            .and(path_regex(
                "/report/reasons/(chapter|manga|scanlation_group|user)",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .report()
            .list()
            .category(ReportCategory::Manga)
            .build()?
            .send()
            .await?;

        assert_eq!(res.response, ResponseType::Collection);
        let reason = &res.data[0];
        assert_eq!(reason.id, report_id);
        assert_eq!(
            reason.attributes.reason.get(&Language::English).unwrap(),
            &"Troll entry".to_string()
        );
        assert!(!reason.attributes.details_required);
        assert_eq!(reason.attributes.category, ReportCategory::Manga);

        Ok(())
    }
}
