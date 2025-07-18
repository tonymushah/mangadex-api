use mangadex_api_types::{MangaDexDateTime, RelationshipType};
use serde::Deserialize;

use crate::TypedAttributes;

#[derive(Clone, Debug, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct UserSettingsTemplateAttributes {
    pub template: serde_json::Value,
    pub created_at: MangaDexDateTime,
    pub updated_at: MangaDexDateTime,
    pub version: u32,
}

impl TypedAttributes for UserSettingsTemplateAttributes {
    const TYPE_: mangadex_api_types::RelationshipType = RelationshipType::SettingsTemplate;
}
