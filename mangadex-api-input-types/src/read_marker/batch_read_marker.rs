#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::manga::id::read::post::MarkChapterBatchBuilder, MangaDexClient};

use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
pub struct MarkChapterBatchParam {
    pub manga_id: Uuid,
    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub chapter_ids_read: Vec<Uuid>,
    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub chapter_ids_unread: Vec<Uuid>,
    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub update_history: bool,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<MarkChapterBatchParam> for MarkChapterBatchBuilder {
    fn from(value: MarkChapterBatchParam) -> Self {
        let mut builder = Self::default();
        builder.manga_id(value.manga_id);
        builder.chapter_ids_read(value.chapter_ids_read);
        builder.chapter_ids_unread(value.chapter_ids_unread);
        builder.update_history(value.update_history);
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl MarkChapterBatchParam {
    pub async fn send(self, client: &MangaDexClient) -> mangadex_api_types::error::Result<()> {
        <MarkChapterBatchBuilder as From<Self>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await?;
        Ok(())
    }
}
