use mangadex_api_types::{MangaDexDateTime, ResultType};
use serde::Deserialize;
use uuid::Uuid;

/// User Settings response.
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
#[allow(unused)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct UserSettingsAttributes {
    #[serde(default)]
    pub result: ResultType,
    #[cfg_attr(feature = "specta", specta(type = String))]
    #[cfg_attr(
        feature = "serialize",
        serde(serialize_with = "crate::v5::mangadex_datetime_serialize")
    )]
    pub updated_at: MangaDexDateTime,
    settings: serde_json::Value,
    pub template: Uuid,
}
