#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::manga::get::ListMangaBuilder, MangaDexClient};

use mangadex_api_types::{
    ContentRating, Demographic, Language, MangaDexDateTime, MangaSortOrder, MangaStatus,
    ReferenceExpansionResource, TagSearchMode,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
#[serde(default)]
pub struct MangaListParams {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub title: Option<String>,
    pub author_or_artist: Option<Uuid>,
    pub authors: Vec<Uuid>,
    pub artists: Vec<Uuid>,
    pub year: Option<u16>,
    pub included_tags: Vec<Uuid>,
    pub included_tags_mode: Option<TagSearchMode>,
    pub excluded_tags: Vec<Uuid>,
    pub excluded_tags_mode: Option<TagSearchMode>,
    pub status: Vec<MangaStatus>,
    /// Languages the manga results are originally published in.
    pub original_language: Vec<Language>,
    /// A list of original languages to exclude.
    pub excluded_original_language: Vec<Language>,
    /// A list of languages that the manga is translated into.
    pub available_translated_language: Vec<Language>,
    pub publication_demographic: Vec<Demographic>,
    pub manga_ids: Vec<Uuid>,
    pub content_rating: Vec<ContentRating>,
    /// DateTime string with following format: `YYYY-MM-DDTHH:MM:SS`.
    pub created_at_since: Option<MangaDexDateTime>,
    /// DateTime string with following format: `YYYY-MM-DDTHH:MM:SS`.
    pub updated_at_since: Option<MangaDexDateTime>,
    pub order: Option<MangaSortOrder>,
    pub includes: Vec<ReferenceExpansionResource>,
    pub has_available_chapters: Option<bool>,
    /// Scanlation group ID.
    pub group: Option<Uuid>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<MangaListParams> for ListMangaBuilder {
    fn from(value: MangaListParams) -> Self {
        let mut builder = Self::default();
        if let Some(limit) = value.limit {
            builder.limit(limit);
        }
        if let Some(offset) = value.offset {
            builder.offset(offset);
        }
        if let Some(title) = value.title {
            builder.title(title);
        }
        if let Some(author_or_artist) = value.author_or_artist {
            builder.author_or_artist(author_or_artist);
        }
        builder.authors(value.authors);
        builder.artists(value.artists);
        if let Some(year) = value.year {
            builder.year(year);
        }
        builder.included_tags(value.included_tags);
        if let Some(included_tags_mode) = value.included_tags_mode {
            builder.included_tags_mode(included_tags_mode);
        }
        builder.excluded_tags(value.excluded_tags);
        if let Some(excluded_tags_mode) = value.excluded_tags_mode {
            builder.excluded_tags_mode(excluded_tags_mode);
        }
        builder.status(value.status);
        builder.original_language(value.original_language);
        builder.excluded_original_language(value.excluded_original_language);
        builder.available_translated_language(value.available_translated_language);
        builder.publication_demographic(value.publication_demographic);
        builder.manga_ids(value.manga_ids);
        builder.content_rating(value.content_rating);
        if let Some(created_at_since) = value.created_at_since {
            builder.created_at_since(created_at_since);
        }
        if let Some(updated_at_since) = value.updated_at_since {
            builder.updated_at_since(updated_at_since);
        }
        if let Some(order) = value.order {
            builder.order(order);
        }
        builder.includes(value.includes);
        if let Some(has_available_chapters) = value.has_available_chapters {
            builder.has_available_chapters(has_available_chapters);
        }
        if let Some(group) = value.group {
            builder.group(group);
        }
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl MangaListParams {
    pub async fn send(self, client: &MangaDexClient) -> mangadex_api_schema::v5::MangaListResponse {
        <ListMangaBuilder as From<Self>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
