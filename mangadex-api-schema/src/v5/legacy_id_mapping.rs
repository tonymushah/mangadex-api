use mangadex_api_types::{LegacyMappingType, RelationshipType};
use serde::Deserialize;
use uuid::Uuid;

use crate::TypedAttributes;

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[non_exhaustive]
pub struct LegacyMappingIdAttributes {
    #[serde(rename = "type")]
    pub type_: Option<LegacyMappingType>,
    pub legacy_id: u32,
    pub new_id: Uuid,
}

impl TypedAttributes for LegacyMappingIdAttributes {
    const TYPE_: mangadex_api_types::RelationshipType = RelationshipType::MappingId;
}
