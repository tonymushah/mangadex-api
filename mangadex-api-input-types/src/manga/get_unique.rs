#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::manga::id::get::GetMangaBuilder, MangaDexClient};

use mangadex_api_types::ReferenceExpansionResource;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct MangaGetUniqueParam {
    manga_id: Uuid,
    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    includes: Vec<ReferenceExpansionResource>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<MangaGetUniqueParam> for GetMangaBuilder {
    fn from(value: MangaGetUniqueParam) -> Self {
        let mut builder = Self::default();
        builder.manga_id(value.manga_id);
        builder.includes(value.includes);
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl MangaGetUniqueParam {
    pub async fn send(
        self,
        client: &MangaDexClient,
    ) -> mangadex_api::Result<mangadex_api_schema::v5::MangaData> {
        let builder: GetMangaBuilder = self.into();
        builder
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
