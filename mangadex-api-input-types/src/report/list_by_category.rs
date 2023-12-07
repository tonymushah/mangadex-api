#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::report::reasons::category::get::ListReasonsBuilder, MangaDexClient};

use mangadex_api_types::ReportCategory;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct ListReasonsByCategory {
    pub category: ReportCategory,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<ListReasonsByCategory> for ListReasonsBuilder {
    fn from(value: ListReasonsByCategory) -> Self {
        let mut builder = Self::default();
        builder.category(value.category);
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl ListReasonsByCategory {
    pub async fn send(
        self,
        client: &MangaDexClient,
    ) -> mangadex_api_schema::v5::ReportReasonListResponse {
        <ListReasonsBuilder as From<ListReasonsByCategory>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
