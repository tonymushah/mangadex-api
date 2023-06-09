use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub type MangaLinks = HashMap<MangaLink, String>;

/// Related link types for a manga.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum MangaLink {
    #[serde(rename = "amz")]
    Amazon,
    #[serde(rename = "al")]
    AniList,
    #[serde(rename = "ap")]
    AnimePlanet,
    #[serde(rename = "bw")]
    BookWalker,
    #[serde(rename = "cdj")]
    CdJapan,
    #[serde(rename = "ebj")]
    EbookJapan,
    #[serde(rename = "engtl")]
    EnglishTranslation,
    #[serde(rename = "kt")]
    Kitsu,
    #[serde(rename = "mu")]
    MangaUpdates,
    #[serde(rename = "mal")]
    MyAnimeList,
    #[serde(rename = "nu")]
    NovelUpdates,
    Raw,
    // TODO: Known issue: Manga ID "f9c33607-9180-4ba6-b85c-e4b5faee7192" has an unknown key of "dj".
    #[serde(other, skip_serializing)]
    Unknown,
}

impl std::fmt::Display for MangaLink {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            Self::Amazon => "amz",
            Self::AniList => "al",
            Self::AnimePlanet => "ap",
            Self::BookWalker => "bw",
            Self::CdJapan => "cdj",
            Self::EbookJapan => "ebj",
            Self::EnglishTranslation => "engtl",
            Self::Kitsu => "kt",
            Self::MangaUpdates => "mu",
            Self::MyAnimeList => "mal",
            Self::NovelUpdates => "nu",
            Self::Raw => "raw",
            Self::Unknown => "<UNKNOWN>",
        })
    }
}
