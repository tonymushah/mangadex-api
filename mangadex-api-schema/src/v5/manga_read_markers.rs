use std::collections::HashMap;

use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[non_exhaustive]
pub enum MangaReadMarkers {
    Ungrouped(UngroupedMangaReadMarkers),
    Grouped(GroupedMangaReadMarkers),
}

#[derive(Debug, Deserialize, Clone, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[non_exhaustive]
pub struct UngroupedMangaReadMarkers {
    pub data: Vec<Uuid>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[non_exhaustive]
pub struct GroupedMangaReadMarkers {
    pub data: HashMap<Uuid, Vec<Uuid>>,
}
