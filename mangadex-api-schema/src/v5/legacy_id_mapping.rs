use mangadex_api_types::LegacyMappingType;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct LegacyMappingIdAttributes {
    #[serde(rename = "type")]
    pub type_: LegacyMappingType,
    pub legacy_id: u64,
    pub new_id: Uuid,
}
