use mangadex_api_types::{MangaDexDateTime, ResultType};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct UserHistoryEntry {
    pub chapter_id: Uuid,
    #[cfg_attr(
        feature = "serialize",
        serde(serialize_with = "crate::v5::mangadex_datetime_serialize")
    )]
    pub read_date: MangaDexDateTime,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct UserHistory {
    #[serde(default)]
    pub result: ResultType,
    pub ratings: Vec<UserHistoryEntry>,
}
