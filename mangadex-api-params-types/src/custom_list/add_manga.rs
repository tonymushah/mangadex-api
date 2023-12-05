#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{
    v5::manga::id::list::list_id::post::AddMangaToCustomListBuilder, MangaDexClient,
};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_types::error::Result;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
pub struct CustomListAddMangaParam {
    pub manga_id: Uuid,
    pub list_id: Uuid,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<CustomListAddMangaParam> for AddMangaToCustomListBuilder {
    fn from(value: CustomListAddMangaParam) -> Self {
        let mut builder = Self::default();
        builder.manga_id(value.manga_id);
        builder.list_id(value.list_id);
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl CustomListAddMangaParam {
    pub async fn send(self, client: &MangaDexClient) -> Result<()> {
        let builder: AddMangaToCustomListBuilder = self.into();
        builder
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
