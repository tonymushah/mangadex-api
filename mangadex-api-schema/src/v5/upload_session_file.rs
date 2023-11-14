//! Upload session file information from a response body.

use mangadex_api_types::{ResultType, UploadSource};
use serde::Deserialize;

use crate::{v5::error::MangaDexError_, FromResponse};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct UploadSessionFileAttributes {
    pub original_file_name: String,
    pub file_hash: String,
    pub file_size: u32,
    pub mime_type: String,
    pub source: UploadSource,
    pub version: u32,
}

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct UploadSessionFileData<A> {
    #[serde(default)]
    pub result: ResultType,
    pub errors: Vec<MangaDexError_>,
    pub data: Vec<A>,
}

impl<A> FromResponse for UploadSessionFileData<A> {
    type Response = Self;
    fn from_response(res: Self::Response) -> Self {
        res
    }
}
