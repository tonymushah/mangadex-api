#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::statistics::chapter::get::FindChapterStatisticsBuilder, MangaDexClient};

use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
pub struct ChaptersStatisticsParams {
    pub chapters: Vec<Uuid>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<ChaptersStatisticsParams> for FindChapterStatisticsBuilder {
    fn from(value: ChaptersStatisticsParams) -> Self {
        let mut builder = Self::default();
        builder.chapter(value.chapters);
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl ChaptersStatisticsParams {
    pub async fn send(
        self,
        client: &MangaDexClient,
    ) -> mangadex_api_schema::v5::ChapterStatisticsResponse {
        <FindChapterStatisticsBuilder as From<Self>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
