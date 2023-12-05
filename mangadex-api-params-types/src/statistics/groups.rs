#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::statistics::group::get::FindGroupStatisticsBuilder, MangaDexClient};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct GroupsStatisticsParams {
    pub groups: Vec<Uuid>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<GroupsStatisticsParams> for FindGroupStatisticsBuilder {
    fn from(value: GroupsStatisticsParams) -> Self {
        let mut builder = Self::default();
        builder.group(value.groups);
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl GroupsStatisticsParams {
    pub async fn send(
        self,
        client: &MangaDexClient,
    ) -> mangadex_api_schema::v5::GroupStatisticsResponse {
        <FindGroupStatisticsBuilder as From<Self>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
