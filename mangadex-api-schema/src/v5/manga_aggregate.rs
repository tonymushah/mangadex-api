use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::v5::{chapter_aggregate_array_or_map, volume_aggregate_array_or_map};

/// Response struct for the manga aggregate endpoint (GET `/manga/:id/aggregate`).
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct MangaAggregate {
    /// Object with (volume_number, volume) key-value pairs.
    #[serde(with = "volume_aggregate_array_or_map")]
    pub volumes: Vec<VolumeAggregate>,
}

#[cfg(feature = "serialize")]
impl Into<MangaAggregatSer> for MangaAggregate {
    fn into(self) -> MangaAggregatSer {
        let mut volumes : HashMap<String, VolumeAggregateSer> = HashMap::new();
        for volume in self.volumes{
            volumes.insert(volume.volume, Into::into(volume));
        }
        MangaAggregatSer { volumes: volumes }
    }
}

#[cfg(feature = "serialize")]
#[derive(serde::Serialize, Clone)]
struct MangaAggregatSer{
    volumes : HashMap<String, VolumeAggregateSer>
}

#[cfg(feature = "serialize")]
#[derive(serde::Serialize, Clone)]
struct VolumeAggregateSer{
    /// Volume number.
    pub volume: String,
    /// Number of chapter translations for the volume.
    pub count: u32,
    /// Object with (chapter_number, chapter) key-value pairs.
    pub chapters: HashMap<String, ChapterAggregate>
}

#[cfg(feature = "serialize")]
impl Serialize for MangaAggregate{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let ser : MangaAggregatSer = Into::into(*self);
        ser.serialize(serializer)
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct VolumeAggregate {
    /// Volume number.
    pub volume: String,
    /// Number of chapter translations for the volume.
    pub count: u32,
    /// Object with (chapter_number, chapter) key-value pairs.
    #[serde(with = "chapter_aggregate_array_or_map")]
    pub chapters: Vec<ChapterAggregate>,
}

#[cfg(feature = "serialize")]
impl Into<VolumeAggregateSer> for VolumeAggregate{
    fn into(self) -> VolumeAggregateSer {
        let mut chapters : HashMap<String, ChapterAggregate> = HashMap::new();
        for chapter in self.chapters {
            chapters.insert(chapter.chapter, chapter);
        }
        VolumeAggregateSer { volume: self.volume, count: self.count, chapters: chapters }
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ChapterAggregate {
    /// Chapter number.
    pub chapter: String,
    pub id: Uuid,
    // TODO: Add docblock explaining what this field represents.
    pub others: Vec<Uuid>,
    /// Number of translations for the chapter.
    pub count: u32,
}
