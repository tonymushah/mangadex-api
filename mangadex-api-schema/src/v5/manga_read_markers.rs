use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum MangaReadMarkers {
    Ungrouped(UngroupedMangaReadMarkers),
    Grouped(GroupedMangaReadMarkers),
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct UngroupedMangaReadMarkers {
    pub data: Vec<Uuid>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GroupedMangaReadMarkers {
    pub data: HashMap<Uuid, Vec<Uuid>>,
}
