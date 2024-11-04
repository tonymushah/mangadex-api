#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::manga::read::get::GetReadChaptersBuilder, MangaDexClient};

use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct ChapterReadMarkersParam {
    pub manga_ids: Vec<Uuid>,
    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub grouped: bool,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<ChapterReadMarkersParam> for GetReadChaptersBuilder {
    fn from(value: ChapterReadMarkersParam) -> Self {
        let mut builder = Self::default();
        builder.manga_ids(value.manga_ids);
        builder.grouped(value.grouped);
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl ChapterReadMarkersParam {
    pub async fn send(
        self,
        client: &MangaDexClient,
    ) -> mangadex_api_schema::v5::MangaReadMarkersResponse {
        <GetReadChaptersBuilder as From<Self>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
