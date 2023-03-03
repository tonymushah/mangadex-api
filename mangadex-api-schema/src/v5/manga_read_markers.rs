use std::collections::HashMap;

use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum MangaReadMarkers {
    Ungrouped(UngroupedMangaReadMarkers),
    Grouped(GroupedMangaReadMarkers),
}

#[derive(Debug, Deserialize, Clone)]
pub struct UngroupedMangaReadMarkers {
    pub data: Vec<Uuid>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GroupedMangaReadMarkers {
    pub data: HashMap<Uuid, Vec<Uuid>>,
}
