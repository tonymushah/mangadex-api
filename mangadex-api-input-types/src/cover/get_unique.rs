#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::cover::cover_id::get::GetCoverBuilder, MangaDexClient};

use mangadex_api_types::ReferenceExpansionResource;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
pub struct CoverGetUniqueParam {
    manga_or_cover_id: Uuid,
    #[serde(default)]
    includes: Vec<ReferenceExpansionResource>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<CoverGetUniqueParam> for GetCoverBuilder {
    fn from(value: CoverGetUniqueParam) -> Self {
        let mut builder = Self::default();
        builder.cover_id(value.manga_or_cover_id);
        builder.includes(value.includes);
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl CoverGetUniqueParam {
    pub async fn send(self, client: &MangaDexClient) -> mangadex_api_schema::v5::CoverResponse {
        let builder: GetCoverBuilder = self.into();
        builder
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
