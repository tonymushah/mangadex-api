use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::v5::{chapter_aggregate_array_or_map, volume_aggregate_array_or_map};

/// Response struct for the manga aggregate endpoint (GET `/manga/:id/aggregate`).
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[non_exhaustive]
pub struct MangaAggregate {
    /// Object with (volume_number, volume) key-value pairs.
    #[serde(with = "volume_aggregate_array_or_map")]
    pub volumes: Vec<VolumeAggregate>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[non_exhaustive]
pub struct VolumeAggregate {
    /// Volume number.
    pub volume: String,
    /// Number of chapter translations for the volume.
    pub count: u32,
    /// Object with (chapter_number, chapter) key-value pairs.
    #[serde(with = "chapter_aggregate_array_or_map")]
    pub chapters: Vec<ChapterAggregate>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ChapterAggregate {
    /// Chapter number.
    pub chapter: String,
    pub id: Uuid,
    // TODO: Add docblock explaining what this field represents.
    pub others: Vec<Uuid>,
    /// Number of translations for the chapter.
    pub count: u32,
}
