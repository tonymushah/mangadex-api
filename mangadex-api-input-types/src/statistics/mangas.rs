#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::statistics::manga::get::FindMangaStatisticsBuilder, MangaDexClient};

use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
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
    ) -> mangadex_api::Result<mangadex_api_schema::v5::MangaStatisticsObject> {
        <FindMangaStatisticsBuilder as From<Self>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
