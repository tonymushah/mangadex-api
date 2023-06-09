use mangadex_api_types::ReadingStatus;
use serde::{Deserialize};

/// Reading status for a single manga.
#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(rename_all = "camelCase")]
pub struct MangaReadingStatus {
    pub status: ReadingStatus,
}
