#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::manga::id::read::post::MarkChapterBatchBuilder, MangaDexClient};

use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
pub struct MarkChapterBatchParam {
    pub manga_id: Uuid,
    #[serde(default)]
    pub chapter_ids_read: Vec<Uuid>,
    #[serde(default)]
    pub chapter_ids_unread: Vec<Uuid>,
    #[serde(default)]
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