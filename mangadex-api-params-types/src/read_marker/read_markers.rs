#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::manga::read::get::GetReadChaptersBuilder, MangaDexClient};

use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
pub struct ChapterReadMarkersParam {
    pub manga_ids: Vec<Uuid>,
    #[serde(default)]
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
