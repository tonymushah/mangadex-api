use std::collections::HashMap;

use mangadex_api_types::ReadingStatus;
use serde::Deserialize;
use uuid::Uuid;

/// Reading statuses for followed manga.
#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct MangaReadingStatuses {
    /// Mapping of manga ID to reading status.
    pub statuses: HashMap<Uuid, ReadingStatus>,
}
