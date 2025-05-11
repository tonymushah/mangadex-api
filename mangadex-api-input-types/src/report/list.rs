#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{
    rate_limit::Limited, v5::report::get::ListReportsByUserBuilder, MangaDexClient,
};

#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::Result;
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_schema::v5::UserReportsCollection;

use mangadex_api_types::{
    ReferenceExpansionResource, ReportCategory, ReportSortOrder, ReportStatus,
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Default, PartialEq, Eq)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(default)]
pub struct ListReportParams {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub category: Option<ReportCategory>,
    pub object_id: Option<Uuid>,
    pub reason_id: Option<Uuid>,
    pub status: Option<ReportStatus>,
    pub order: Option<ReportSortOrder>,
    #[cfg_attr(feature = "async-graphql", graphql(skip))]
    pub includes: Vec<ReferenceExpansionResource>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<ListReportParams> for ListReportsByUserBuilder {
    fn from(value: ListReportParams) -> Self {
        let mut builder = Self::default();
        if let Some(limit) = value.limit {
            builder.limit(limit);
        }
        if let Some(offset) = value.offset {
            builder.offset(offset);
        }
        if let Some(category) = value.category {
            builder.category(category);
        }
        if let Some(object_id) = value.object_id {
            builder.object_id(object_id);
        }
        if let Some(reason_id) = value.reason_id {
            builder.reason_id(reason_id);
        }
        if let Some(status) = value.status {
            builder.status(status);
        }
        if let Some(order) = value.order {
            builder.order(order);
        }
        builder.includes(value.includes);
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl ListReportParams {
    pub async fn send(self, client: &MangaDexClient) -> Result<Limited<UserReportsCollection>> {
        <ListReportsByUserBuilder as From<Self>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
