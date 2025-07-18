//! Upload session information from a response body.

use mangadex_api_types::{MangaDexDateTime, RelationshipType};
use serde::Deserialize;

use crate::TypedAttributes;

#[derive(Clone, Debug, Deserialize, Copy, Default)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct UploadSessionAttributes {
    pub is_committed: bool,
    pub is_processed: bool,
    pub is_deleted: bool,
    pub version: u32,
    /// Datetime in `YYYY-MM-DDTHH:MM:SS+HH:MM` format.
    #[cfg_attr(feature = "specta", specta(type = String))]
    #[cfg_attr(
        feature = "serialize",
        serde(serialize_with = "crate::v5::mangadex_datetime_serialize")
    )]
    pub created_at: MangaDexDateTime,
    /// Datetime in `YYYY-MM-DDTHH:MM:SS+HH:MM` format.
    #[cfg_attr(feature = "specta", specta(type = String))]
    #[cfg_attr(
        feature = "serialize",
        serde(serialize_with = "crate::v5::mangadex_datetime_serialize")
    )]
    pub updated_at: MangaDexDateTime,
}

impl TypedAttributes for UploadSessionAttributes {
    const TYPE_: mangadex_api_types::RelationshipType = RelationshipType::UploadSession;
}
