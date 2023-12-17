#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::manga::draft::id::get::GetMangaDraftBuilder, MangaDexClient};

use mangadex_api_types::ReferenceExpansionResource;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
pub struct GetMangaDraftParams {
    pub manga_id: Uuid,

    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub includes: Vec<ReferenceExpansionResource>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<GetMangaDraftParams> for GetMangaDraftBuilder {
    fn from(value: GetMangaDraftParams) -> Self {
        let mut builder = Self::default();
        builder.manga_id(value.manga_id);
        builder.includes(value.includes);
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl GetMangaDraftParams {
    pub async fn send(self, client: &MangaDexClient) -> mangadex_api_schema::v5::MangaResponse {
        let builder: GetMangaDraftBuilder = self.into();
        builder
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
