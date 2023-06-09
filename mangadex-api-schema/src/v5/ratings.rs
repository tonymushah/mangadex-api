//! Manga statistics from a response body.

use std::collections::HashMap;

use mangadex_api_types::MangaDexDateTime;
use serde::{Deserialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct RatingsList {
    pub ratings: HashMap<Uuid, Rating>,
}

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct Rating {
    /// `[ 1 .. 10 ]`.
    pub rating: u8,
    /// Datetime in `YYYY-MM-DDTHH:MM:SS+HH:MM` format.
    pub created_at: MangaDexDateTime,
}
