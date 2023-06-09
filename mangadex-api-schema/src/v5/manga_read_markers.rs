use std::collections::HashMap;

use serde::{Deserialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MangaReadMarkers {
    Ungrouped(UngroupedMangaReadMarkers),
    Grouped(GroupedMangaReadMarkers),
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct UngroupedMangaReadMarkers {
    pub data: Vec<Uuid>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct GroupedMangaReadMarkers {
    pub data: HashMap<Uuid, Vec<Uuid>>,
}
