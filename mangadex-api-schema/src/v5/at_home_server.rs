use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AtHomeServer {
    /// The base URL to construct final image URLs from.
    /// The URL returned is valid for the requested chapter only, and for a duration of 15 minutes
    /// from the time of the response.
    pub base_url: Url,
    pub chapter: ChapterData,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ChapterData {
    pub hash: String,
    /// Original upload quality filenames.
    pub data: Vec<String>,
    /// Compressed quality filenames.
    pub data_saver: Vec<String>,
}
