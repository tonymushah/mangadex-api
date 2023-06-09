//! Upload session information from a response body.

use mangadex_api_types::{MangaDexDateTime, RelationshipType};
use serde::{Deserialize};
use uuid::Uuid;

use crate::FromResponse;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct UploadSessionAttributes {
    pub is_committed: bool,
    pub is_processed: bool,
    pub is_deleted: bool,
    pub version: u32,
    /// Datetime in `YYYY-MM-DDTHH:MM:SS+HH:MM` format.
    #[cfg_attr(feature = "specta", specta(type = String))]
    pub created_at: MangaDexDateTime,
    /// Datetime in `YYYY-MM-DDTHH:MM:SS+HH:MM` format.
    #[cfg_attr(feature = "specta", specta(type = String))]
    pub updated_at: MangaDexDateTime,
}

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct UploadSessionResponse {
    pub id: Uuid,
    #[serde(rename = "type")]
    pub type_: RelationshipType,
    pub attributes: UploadSessionAttributes,
}

impl FromResponse for UploadSessionResponse {
    type Response = Self;

    fn from_response(value: Self::Response) -> Self {
        value
    }
}
