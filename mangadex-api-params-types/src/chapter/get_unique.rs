#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::chapter::id::get::GetChapterBuilder, MangaDexClient};

use mangadex_api_types::ReferenceExpansionResource;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ChapterGetUniqueParam {
    pub id: Uuid,
    #[serde(default)]
    pub includes: Vec<ReferenceExpansionResource>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<ChapterGetUniqueParam> for GetChapterBuilder {
    fn from(value: ChapterGetUniqueParam) -> Self {
        let mut builder = Self::default();
        builder.chapter_id(value.id);
        builder.includes(value.includes);
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl ChapterGetUniqueParam {
    pub async fn send(self, client: &MangaDexClient) -> mangadex_api_schema::v5::ChapterResponse {
        let builder: GetChapterBuilder = self.into();
        builder
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
