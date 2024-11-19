use mangadex_api_types::MangaDexDateTime;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct UserSettingsTemplateAttributes {
    pub template: serde_json::Value,
    pub created_at: MangaDexDateTime,
    pub updated_at: MangaDexDateTime,
    pub version: u32,
}
