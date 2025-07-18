use std::collections::HashMap;

use mangadex_api_types::ResultType;
use serde::Deserialize;
use uuid::Uuid;

use super::Comments;

#[derive(Clone, Debug, Deserialize, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct GroupStatisticsObject {
    #[serde(default)]
    pub result: ResultType,
    /// JSON object of `MangaId-StatisticsObject`.
    pub statistics: HashMap<Uuid, GroupStatistics>,
}

#[derive(Clone, Debug, Deserialize, Copy, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[non_exhaustive]
pub struct GroupStatistics {
    pub comments: Option<Comments>,
}
