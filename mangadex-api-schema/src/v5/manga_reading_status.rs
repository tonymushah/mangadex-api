use mangadex_api_types::{ReadingStatus, ResultType};
use serde::Deserialize;

/// Reading status for a single manga.
#[derive(Debug, Deserialize, Clone, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[non_exhaustive]
pub struct MangaReadingStatus {
    #[serde(default)]
    pub result: ResultType,
    pub status: Option<ReadingStatus>,
}
