use mangadex_api_types::ResultType;
use serde::Deserialize;
use url::Url;

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct AtHomeServer {
    #[serde(default)]
    pub result : ResultType,
    /// The base URL to construct final image URLs from.
    /// The URL returned is valid for the requested chapter only, and for a duration of 15 minutes
    /// from the time of the response.
    pub base_url: Url,
    pub chapter: ChapterData,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ChapterData {
    pub hash: String,
    /// Original upload quality filenames.
    pub data: Vec<String>,
    /// Compressed quality filenames.
    pub data_saver: Vec<String>,
}
