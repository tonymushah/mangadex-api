//! Upload session file information from a response body.

use mangadex_api_types::UploadSource;
use serde::Deserialize;

use crate::v5::error::MangaDexError;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct UploadSessionFileAttributes {
    pub original_file_name: String,
    pub file_hash: String,
    pub file_size: u64,
    pub mime_type: String,
    pub source: UploadSource,
    pub version: u32,
}

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct UploadSessionFileData<A> {
    pub errors: Vec<MangaDexError>,
    pub data: Vec<A>,
}
