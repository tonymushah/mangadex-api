#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{
    v5::manga::manga_id::relation::post::CreateMangaRelationBuilder, MangaDexClient,
};

use mangadex_api_types::MangaRelation;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
pub struct MangaCreateRelationParam {
    pub manga_id: Uuid,
    pub target_manga: Uuid,
    pub relation: MangaRelation,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<MangaCreateRelationParam> for CreateMangaRelationBuilder {
    fn from(value: MangaCreateRelationParam) -> Self {
        let mut builder = Self::default();
        builder.manga_id(value.manga_id);
        builder.target_manga(value.target_manga);
        builder.relation(value.relation);
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl MangaCreateRelationParam {
    pub async fn send(
        self,
        client: &MangaDexClient,
    ) -> mangadex_api::Result<mangadex_api_schema::v5::MangaRelationCollection> {
        <CreateMangaRelationBuilder as From<Self>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
