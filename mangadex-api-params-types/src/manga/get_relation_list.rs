#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::manga::manga_id::relation::get::ListMangaRelationsBuilder, MangaDexClient};

use mangadex_api_types::ReferenceExpansionResource;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
pub struct MangaRelationParam {
    manga_id: Uuid,
    #[serde(default)]
    includes: Vec<ReferenceExpansionResource>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<MangaRelationParam> for ListMangaRelationsBuilder {
    fn from(value: MangaRelationParam) -> Self {
        let mut builder = Self::default();
        builder.manga_id(value.manga_id);
        builder.includes(value.includes);
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl MangaRelationParam {
    pub async fn send(
        self,
        client: &MangaDexClient,
    ) -> mangadex_api_schema::v5::MangaRelationListResponse {
        let builder: ListMangaRelationsBuilder = self.into();
        builder
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
