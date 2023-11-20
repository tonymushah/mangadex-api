#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{
    v5::manga::id::list::list_id::delete::RemoveMangaFromCustomListBuilder, MangaDexClient,
};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_types::error::Result;

use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
pub struct CustomListRemoveMangaParam {
    pub manga_id: Uuid,
    pub list_id: Uuid,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<CustomListRemoveMangaParam> for RemoveMangaFromCustomListBuilder {
    fn from(value: CustomListRemoveMangaParam) -> Self {
        let mut builder = Self::default();
        builder.manga_id(value.manga_id);
        builder.list_id(value.list_id);
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl CustomListRemoveMangaParam {
    pub async fn send(self, client: &MangaDexClient) -> Result<()> {
        let builder: RemoveMangaFromCustomListBuilder = self.into();
        builder
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
