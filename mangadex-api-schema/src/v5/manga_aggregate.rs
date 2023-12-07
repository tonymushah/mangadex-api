#[cfg(feature = "serialize")]
use std::collections::HashMap;

use mangadex_api_types::ResultType;
#[cfg(feature = "serialize")]
use serde::Serialize;

use serde::Deserialize;
use uuid::Uuid;

use crate::v5::{chapter_aggregate_array_or_map, volume_aggregate_array_or_map};

/// Response struct for the manga aggregate endpoint (GET `/manga/:id/aggregate`).
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
pub struct MangaAggregate {
    #[serde(default)]
    pub result: ResultType,
    /// Object with (volume_number, volume) key-value pairs.
    #[serde(with = "volume_aggregate_array_or_map")]
    pub volumes: Vec<VolumeAggregate>,
}

#[cfg(feature = "specta")]
impl specta::Type for MangaAggregate {
    fn inline(
        opts: specta::DefOpts,
        generics: &[specta::DataType],
    ) -> Result<specta::DataType, specta::ExportError> {
        MangaAggregatSer::inline(opts, generics)
    }
}

#[cfg(feature = "serialize")]
impl From<MangaAggregate> for MangaAggregatSer {
    fn from(value: MangaAggregate) -> Self {
        let mut volumes: HashMap<String, VolumeAggregateSer> = HashMap::new();
        for volume in value.volumes {
            volumes.insert(volume.volume.clone(), Into::into(volume.clone()));
        }
        MangaAggregatSer {
            result: value.result,
            volumes,
        }
    }
}

#[cfg(feature = "serialize")]
#[derive(serde::Serialize, Clone)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct MangaAggregatSer {
    result: ResultType,
    volumes: HashMap<String, VolumeAggregateSer>,
}

#[cfg(feature = "serialize")]
#[derive(serde::Serialize, Clone)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct VolumeAggregateSer {
    /// Volume number.
    pub volume: String,
    /// Number of chapter translations for the volume.
    pub count: u32,
    /// Object with (chapter_number, chapter) key-value pairs.
    pub chapters: HashMap<String, ChapterAggregate>,
}

#[cfg(feature = "serialize")]
impl Serialize for MangaAggregate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let ser: MangaAggregatSer = Into::into(self.clone());
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
#[allow(clippy::from_over_into)]
impl From<VolumeAggregate> for VolumeAggregateSer {
    fn from(value: VolumeAggregate) -> Self {
        let mut chapters: HashMap<String, ChapterAggregate> = HashMap::new();
        for chapter in value.chapters {
            chapters.insert(chapter.chapter.clone(), chapter);
        }
        VolumeAggregateSer {
            volume: value.volume,
            count: value.count,
            chapters,
        }
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
