//! Manga statistics from a response body.

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
pub struct MangaStatisticsObject {
    #[serde(default)]
    pub result: ResultType,
    /// JSON object of `MangaId-StatisticsObject`.
    pub statistics: HashMap<Uuid, MangaStatistics>,
}

#[derive(Clone, Debug, Deserialize, Copy)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[non_exhaustive]
pub struct MangaStatistics {
    pub rating: MangaRating,
    /// Number of users following the Manga.
    // The API documentation has placed this within the `rating` object as of MangaDex API 5.4.9 but
    // the actual response has this field at this level.
    pub follows: u32,
    pub comments: Option<Comments>,
    #[serde(default)]
    pub unavailable_chapter_count: u32,
}

#[derive(Clone, Debug, Deserialize, Copy)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[non_exhaustive]
pub struct MangaRating {
    /// Average rating of distributed votes.
    ///
    /// Ratings values with no votes are not included in the calculation.
    ///
    /// Will be `None` if no ratings calculations have been done.
    #[serde(default)]
    pub average: Option<f32>,
    #[serde(default)]
    pub bayesian: Option<f32>,
    /// Ordered distribution of ratings from 1 to 10.
    ///
    /// Array indices correspond to the rating value.
    ///
    /// Each element corresponds to the number of users that have given that rating.
    #[serde(default)]
    #[cfg_attr(feature = "specta", specta(type = HashMap<String, u32>))]
    pub distribution: RatingsDistribution,
}

/// Distribution of ratings from 1 to 10.
///
/// Because Rust identifies may not begin with a number, the fields are prefixed with an arbitrary
/// "r" to denote "rating".
#[derive(Clone, Debug, Deserialize, Default, Copy)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[non_exhaustive]
pub struct RatingsDistribution {
    #[serde(rename = "1")]
    pub r1: u32,
    #[serde(rename = "2")]
    pub r2: u32,
    #[serde(rename = "3")]
    pub r3: u32,
    #[serde(rename = "4")]
    pub r4: u32,
    #[serde(rename = "5")]
    pub r5: u32,
    #[serde(rename = "6")]
    pub r6: u32,
    #[serde(rename = "7")]
    pub r7: u32,
    #[serde(rename = "8")]
    pub r8: u32,
    #[serde(rename = "9")]
    pub r9: u32,
    #[serde(rename = "10")]
    pub r10: u32,
}
