#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::manga::id::aggregate::get::GetMangaAggregateBuilder, MangaDexClient};

use mangadex_api_types::Language;
use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
pub struct MangaAggregateParam {
    pub manga_id: Uuid,

    #[serde(default)]
    pub translated_language: Vec<Language>,
    #[serde(default)]
    pub groups: Vec<Uuid>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<MangaAggregateParam> for GetMangaAggregateBuilder {
    fn from(value: MangaAggregateParam) -> Self {
        let mut builder = Self::default();
        builder.manga_id(value.manga_id);
        builder.translated_language(value.translated_language);
        builder.groups(value.groups);
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl MangaAggregateParam {
    pub async fn send(
        self,
        client: &MangaDexClient,
    ) -> mangadex_api_schema::v5::MangaAggregateResponse {
        <GetMangaAggregateBuilder as From<Self>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
