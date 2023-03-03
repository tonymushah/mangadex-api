use serde::Deserialize;
use url::Url;
use uuid::Uuid;

use crate::deserialize_null_default;
use mangadex_api_types::{Language, MangaDexDateTime};

/// General chapter information.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ChapterAttributes {
    // TODO: Known issue: API doesn't always return an empty string despite the docs saying it's not nullable.
    #[serde(deserialize_with = "deserialize_null_default")]
    pub title: String,
    /// Volume number in the manga.
    pub volume: Option<String>,
    /// Chapter number in the manga.
    pub chapter: Option<String>,
    /// Count of readable images for this chapter.
    pub pages: u32,
    /// Language the text is in.
    pub translated_language: Language,
    /// User ID (UUID) who uploaded the chapter.
    pub uploader: Option<Uuid>,
    /// Denotes a chapter that links to an external source.
    pub external_url: Option<Url>,
    pub version: u32,
    /// Datetime in `YYYY-MM-DDTHH:MM:SS+HH:MM` format.
    pub created_at: MangaDexDateTime,
    /// Datetime in `YYYY-MM-DDTHH:MM:SS+HH:MM` format.
    pub updated_at: Option<MangaDexDateTime>,
    /// Datetime in `YYYY-MM-DDTHH:MM:SS+HH:MM` format.
    pub publish_at: MangaDexDateTime,
    /// Datetime in `YYYY-MM-DDTHH:MM:SS+HH:MM` format.
    pub readable_at: MangaDexDateTime,
}
