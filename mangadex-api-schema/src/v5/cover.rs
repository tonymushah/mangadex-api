use mangadex_api_types::{Language, MangaDexDateTime, RelationshipType};
use serde::Deserialize;

use crate::TypedAttributes;

/// General cover information.
#[derive(Clone, Debug, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct CoverAttributes {
    pub description: String,
    pub locale: Option<Language>,
    /// Volume number in the manga.
    pub volume: Option<String>,
    /// Cover art filename as it's stored on the MangaDex servers.
    pub file_name: String,
    /// Datetime in `YYYY-MM-DDTHH:MM:SS+HH:MM` format.
    #[cfg_attr(feature = "specta", specta(type = String))]
    #[cfg_attr(
        feature = "serialize",
        serde(serialize_with = "crate::v5::mangadex_datetime_serialize")
    )]
    pub created_at: MangaDexDateTime,
    /// Datetime in `YYYY-MM-DDTHH:MM:SS+HH:MM` format.
    #[cfg_attr(feature = "specta", specta(type = Option<String>))]
    #[cfg_attr(
        feature = "serialize",
        serde(serialize_with = "crate::v5::mangadex_datetime_serialize_option")
    )]
    pub updated_at: Option<MangaDexDateTime>,
    pub version: u32,
}

impl TypedAttributes for CoverAttributes {
    const TYPE_: mangadex_api_types::RelationshipType = RelationshipType::CoverArt;
}
