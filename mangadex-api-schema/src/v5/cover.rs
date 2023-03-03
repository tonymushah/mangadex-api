use mangadex_api_types::{Language, MangaDexDateTime};
use serde::Deserialize;

/// General cover information.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CoverAttributes {
    pub description: String,
    pub locale: Option<Language>,
    /// Volume number in the manga.
    pub volume: Option<String>,
    /// Cover art filename as it's stored on the MangaDex servers.
    pub file_name: String,
    /// Datetime in `YYYY-MM-DDTHH:MM:SS+HH:MM` format.
    pub created_at: MangaDexDateTime,
    /// Datetime in `YYYY-MM-DDTHH:MM:SS+HH:MM` format.
    pub updated_at: Option<MangaDexDateTime>,
    pub version: u32,
}
