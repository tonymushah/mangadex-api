//! Upload session information from a response body.

use mangadex_api_types::{MangaDexDateTime, RelationshipType};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::FromResponse;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct UploadSessionAttributes {
    pub is_committed: bool,
    pub is_processed: bool,
    pub is_deleted: bool,
    pub version: u32,
    /// Datetime in `YYYY-MM-DDTHH:MM:SS+HH:MM` format.
    pub created_at: MangaDexDateTime,
    /// Datetime in `YYYY-MM-DDTHH:MM:SS+HH:MM` format.
    pub updated_at: MangaDexDateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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
