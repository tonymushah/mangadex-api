#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::report::post::CreateReportBuilder, MangaDexClient};

use mangadex_api_types::ReportCategory;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct CreateReportParam {
    pub category: ReportCategory,
    /// The report reason ID for sub-categorization.
    ///
    /// For example, if a manga was being reported for being a troll entry, the specific reason ID should be used, obtained from the [list report reasons endpoint](crate::v5::report::list).
    pub reason: Uuid,
    /// The ID from the category type.
    ///
    /// For example, if the category is "manga", this should be a manga UUID.
    pub object_id: Uuid,
    /// Optional notes about why this is being reported.
    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub details: Option<String>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<CreateReportParam> for CreateReportBuilder {
    fn from(value: CreateReportParam) -> Self {
        let mut builder = Self::default();
        builder.category(value.category);
        builder.reason(value.reason);
        builder.object_id(value.object_id);
        if let Some(details) = value.details {
            builder.details(details);
        }
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl CreateReportParam {
    pub async fn send(
        self,
        client: &MangaDexClient,
    ) -> mangadex_api_types::error::Result<mangadex_api_schema::Limited<()>> {
        let res = <CreateReportBuilder as From<Self>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await?;
        Ok(mangadex_api_schema::Limited {
            rate_limit: res.rate_limit,
            body: (),
        })
    }
}
