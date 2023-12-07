#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::statistics::chapter::get::FindChapterStatisticsBuilder, MangaDexClient};

use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
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
