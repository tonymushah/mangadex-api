#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::manga::id::aggregate::get::GetMangaAggregateBuilder, MangaDexClient};

use mangadex_api_types::Language;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
pub struct MangaAggregateParam {
    pub manga_id: Uuid,

    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub translated_language: Vec<Language>,
    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
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
