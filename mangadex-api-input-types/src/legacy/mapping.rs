#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::legacy::mapping::post::LegacyIdMappingBuilder, MangaDexClient};

use mangadex_api_types::LegacyMappingType;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
pub struct LegacyIdMappingParams {
    pub map_type: LegacyMappingType,
    pub ids: Vec<u32>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<LegacyIdMappingParams> for LegacyIdMappingBuilder {
    fn from(value: LegacyIdMappingParams) -> Self {
        let mut builder = Self::default();
        builder.map_type(value.map_type);
        builder.ids(value.ids);
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl LegacyIdMappingParams {
    pub async fn send(
        self,
        client: &MangaDexClient,
    ) -> mangadex_api::Result<mangadex_api_schema::v5::IdMappingCollection> {
        <LegacyIdMappingBuilder as From<Self>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
