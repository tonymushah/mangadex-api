//! Upload session file information from a response body.

use mangadex_api_types::{RelationshipType, ResultType, UploadSource};
use serde::Deserialize;

use crate::{TypedAttributes, error::MangaDexError};

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[non_exhaustive]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct UploadSessionFileAttributes {
    pub original_file_name: String,
    pub file_hash: String,
    pub file_size: u32,
    pub mime_type: String,
    pub source: UploadSource,
    pub version: u32,
}

impl TypedAttributes for UploadSessionFileAttributes {
    const TYPE_: mangadex_api_types::RelationshipType = RelationshipType::UploadSessionFile;
}

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[non_exhaustive]
pub struct UploadSessionFileData<A> {
    #[serde(default)]
    pub result: ResultType,
    pub errors: Vec<MangaDexError>,
    pub data: Vec<A>,
}

impl<A> Default for UploadSessionFileData<A> {
    fn default() -> Self {
        Self {
            result: ResultType::Ok,
            errors: Vec::default(),
            data: Vec::default(),
        }
    }
}
