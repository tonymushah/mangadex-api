#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::manga::draft::id::commit::post::SubmitMangaDraftBuilder, MangaDexClient};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_schema::{v5::MangaData, Limited};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_types::error::Result;

use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct SubmitMangaDraftParams {
    pub manga_id: Uuid,
    pub version: u32,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<SubmitMangaDraftParams> for SubmitMangaDraftBuilder {
    fn from(value: SubmitMangaDraftParams) -> Self {
        let mut builder = Self::default();
        builder.manga_id(value.manga_id);
        builder.version(value.version);
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl SubmitMangaDraftParams {
    pub async fn send(self, client: &MangaDexClient) -> Result<Limited<MangaData>> {
        <SubmitMangaDraftBuilder as From<Self>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
