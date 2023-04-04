use std::collections::HashMap;

use mangadex_api_types::ReadingStatus;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Reading statuses for followed manga.
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MangaReadingStatuses {
    /// Mapping of manga ID to reading status.
    pub statuses: HashMap<Uuid, ReadingStatus>,
}
