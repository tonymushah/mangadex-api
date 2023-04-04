//! Manga statistics from a response body.

use std::collections::HashMap;

use mangadex_api_types::MangaDexDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RatingsList {
    pub ratings: HashMap<Uuid, Rating>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Rating {
    /// `[ 1 .. 10 ]`.
    pub rating: u8,
    /// Datetime in `YYYY-MM-DDTHH:MM:SS+HH:MM` format.
    pub created_at: MangaDexDateTime,
}
