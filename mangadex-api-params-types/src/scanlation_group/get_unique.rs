#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::scanlation_group::id::get::GetGroupBuilder, MangaDexClient};

use mangadex_api_types::ReferenceExpansionResource;
use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
pub struct ScanlationGroupGetUniqueParam {
    group_id: Uuid,
    #[serde(default)]
    includes: Vec<ReferenceExpansionResource>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<ScanlationGroupGetUniqueParam> for GetGroupBuilder {
    fn from(value: ScanlationGroupGetUniqueParam) -> Self {
        let mut builder = Self::default();
        builder.group_id(value.group_id);
        builder.includes(value.includes);
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl ScanlationGroupGetUniqueParam {
    pub async fn send(self, client: &MangaDexClient) -> mangadex_api_schema::v5::GroupResponse {
        let builder: GetGroupBuilder = self.into();
        builder
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
