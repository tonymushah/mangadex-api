use mangadex_api_types::LegacyMappingType;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LegacyMappingIdAttributes {
    #[serde(rename = "type")]
    pub type_: LegacyMappingType,
    pub legacy_id: u64,
    pub new_id: Uuid,
}
