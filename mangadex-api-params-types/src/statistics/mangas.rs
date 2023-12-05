#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::statistics::manga::get::FindMangaStatisticsBuilder, MangaDexClient};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct MangasStatisticsParams {
    pub mangas: Vec<Uuid>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<MangasStatisticsParams> for FindMangaStatisticsBuilder {
    fn from(value: MangasStatisticsParams) -> Self {
        let mut builder = Self::default();
        builder.manga(value.mangas);
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl MangasStatisticsParams {
    pub async fn send(
        self,
        client: &MangaDexClient,
    ) -> mangadex_api_schema::v5::MangaStatisticsResponse {
        <FindMangaStatisticsBuilder as From<Self>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
