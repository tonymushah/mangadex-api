use mangadex_api_types::ReadingStatus;
use serde::{Deserialize, Serialize};

/// Reading status for a single manga.
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MangaReadingStatus {
    pub status: ReadingStatus,
}
