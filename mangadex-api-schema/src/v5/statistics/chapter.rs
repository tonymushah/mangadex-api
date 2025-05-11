use std::collections::HashMap;

use mangadex_api_types::ResultType;
use serde::Deserialize;
use uuid::Uuid;

use super::Comments;

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ChapterStatisticsObject {
    #[serde(default)]
    pub result: ResultType,
    /// JSON object of `MangaId-StatisticsObject`.
    pub statistics: HashMap<Uuid, ChapterStatistics>,
}

#[derive(Clone, Debug, Deserialize, Copy)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ChapterStatistics {
    pub comments: Option<Comments>,
}
