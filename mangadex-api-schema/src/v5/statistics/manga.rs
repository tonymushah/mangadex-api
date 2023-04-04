//! Manga statistics from a response body.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::FromResponse;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct MangaStatisticsObject {
    /// JSON object of `MangaId-StatisticsObject`.
    pub statistics: HashMap<Uuid, MangaStatistics>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MangaStatistics {
    pub rating: MangaRating,
    /// Number of users following the Manga.
    // The API documentation has placed this within the `rating` object as of MangaDex API 5.4.9 but
    // the actual response has this field at this level.
    pub follows: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MangaRating {
    /// Average rating of distributed votes.
    ///
    /// Ratings values with no votes are not included in the calculation.
    ///
    /// Will be `None` if no ratings calculations have been done.
    pub average: Option<f32>,
    /// Ordered distribution of ratings from 1 to 10.
    ///
    /// Array indices correspond to the rating value.
    ///
    /// Each element corresponds to the number of users that have given that rating.
    #[serde(default)]
    pub distribution: RatingsDistribution,
}

/// Distribution of ratings from 1 to 10.
///
/// Because Rust identifies may not begin with a number, the fields are prefixed with an arbitrary
/// "r" to denote "rating".
#[derive(Clone, Debug, Deserialize, Default, Serialize)]
pub struct RatingsDistribution {
    #[serde(rename = "1")]
    pub r1: u64,
    #[serde(rename = "2")]
    pub r2: u64,
    #[serde(rename = "3")]
    pub r3: u64,
    #[serde(rename = "4")]
    pub r4: u64,
    #[serde(rename = "5")]
    pub r5: u64,
    #[serde(rename = "6")]
    pub r6: u64,
    #[serde(rename = "7")]
    pub r7: u64,
    #[serde(rename = "8")]
    pub r8: u64,
    #[serde(rename = "9")]
    pub r9: u64,
    #[serde(rename = "10")]
    pub r10: u64,
}

impl FromResponse for MangaStatisticsObject {
    type Response = Self;

    fn from_response(value: Self::Response) -> Self {
        value
    }
}
