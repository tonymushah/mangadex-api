use mangadex_api_types::{
    ContentRating, Demographic, Language, MangaDexDateTime, MangaState, MangaStatus,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::v5::{
    language_array_or_skip_null, localizedstring_array_or_map, manga_links_array_or_struct,
    ApiObject, LocalizedString, MangaLinks, TagAttributes,
};

/// General manga information.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct MangaAttributes {
    pub title: LocalizedString,
    pub alt_titles: Vec<LocalizedString>,
    #[serde(with = "localizedstring_array_or_map")]
    pub description: LocalizedString,
    // Known issue: This field isn't always returned, so default to `false` when it isn't.
    // The decision to use the default value is to maintain compatibility if the MangaDex API
    // fixes this by always returning this field.
    #[serde(default)]
    pub is_locked: bool,
    #[serde(with = "manga_links_array_or_struct")]
    pub links: Option<MangaLinks>,
    pub original_language: Language,
    pub last_volume: Option<String>,
    pub last_chapter: Option<String>,
    pub publication_demographic: Option<Demographic>,
    pub status: MangaStatus,
    pub year: Option<u16>,
    pub content_rating: Option<ContentRating>,
    // Known issue: This field isn't always returned, so default to `false` when it isn't.
    // TODO: Remove the default when MangaDex always returns this field.
    #[serde(default)]
    pub chapter_numbers_reset_on_new_volume: bool,
    // Known issue: MangaDex sometimes returns `null` as an element value, which doesn't match a possible language.
    #[serde(with = "language_array_or_skip_null")]
    pub available_translated_languages: Vec<Language>,
    pub tags: Vec<ApiObject<TagAttributes>>,
    /// The staff approval status of the manga.
    ///
    /// When a new manga is created with the Manga Create endpoint, it is in a "draft" state.
    /// When it is submitted (committed), it must be approved (published) or rejected by staff.
    ///
    /// Manga that is in the "draft" state is not available through the search,
    /// however, endpoints to list or retrieve the Manga Drafts are available.
    pub state: MangaState,
    /// Datetime in `YYYY-MM-DDTHH:MM:SS+HH:MM` format.
    #[cfg_attr(feature = "specta", specta(type = String))]
    pub created_at: MangaDexDateTime,
    /// Datetime in `YYYY-MM-DDTHH:MM:SS+HH:MM` format.
    #[cfg_attr(feature = "specta", specta(type = Option<String>))]
    pub updated_at: Option<MangaDexDateTime>,
    pub version: u32,
    pub latest_uploaded_chapter : Option<Uuid>
}
