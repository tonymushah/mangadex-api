use std::collections::HashMap;

use mangadex_api_types::ResultType;
use serde::Deserialize;
use uuid::Uuid;

use super::Comments;

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct GroupStatisticsObject {
    pub result : ResultType,
    /// JSON object of `MangaId-StatisticsObject`.
    pub statistics: HashMap<Uuid, GroupStatistics>,
}

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct GroupStatistics {
    pub comments : Option<Comments>
}
