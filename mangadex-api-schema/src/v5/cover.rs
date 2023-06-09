use mangadex_api_types::{Language, MangaDexDateTime};
use serde::{Deserialize};

/// General cover information.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
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
    pub created_at: MangaDexDateTime,
    /// Datetime in `YYYY-MM-DDTHH:MM:SS+HH:MM` format.
    #[cfg_attr(feature = "specta", specta(type = Option<String>))]
    pub updated_at: Option<MangaDexDateTime>,
    pub version: u32,
}
