#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::manga::id::put::UpdateMangaBuilder, MangaDexClient};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_schema::{v5::MangaData, Limited};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_types::error::Result;

use mangadex_api_schema::v5::LocalizedString;
use mangadex_api_types::{ContentRating, Demographic, Language, MangaLinks, MangaStatus};
use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
pub struct UpdateMangaParam {
    pub manga_id: Uuid,
    pub title: LocalizedString,
    #[serde(default)]
    pub alt_titles: Option<Vec<LocalizedString>>,
    #[serde(default)]
    pub description: Option<LocalizedString>,
    #[serde(default)]
    pub authors: Option<Vec<Uuid>>,
    #[serde(default)]
    pub artists: Option<Vec<Uuid>>,
    #[serde(default)]
    pub links: Option<MangaLinks>,
    pub original_language: Language,
    #[serde(default)]
    pub last_volume: Option<String>,
    #[serde(default)]
    pub last_chapter: Option<String>,
    #[serde(default)]
    pub publication_demographic: Option<Option<Demographic>>,
    pub status: MangaStatus,
    /// Year the manga was released.
    #[serde(default)]
    pub year: Option<Option<u16>>,
    pub content_rating: ContentRating,
    #[serde(default)]
    pub chapter_numbers_reset_on_new_volume: Option<bool>,
    #[serde(default)]
    pub tags: Option<Vec<Uuid>>,
    /// Cover ID.
    #[serde(default)]
    pub primary_cover: Option<Option<Uuid>>,
    /// >= 1
    pub version: u32,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<UpdateMangaParam> for UpdateMangaBuilder {
    fn from(value: UpdateMangaParam) -> Self {
        let mut builder = Self::default();
        builder.manga_id(value.manga_id);
        builder.title(value.title);
        if let Some(alt_titles) = value.alt_titles {
            builder.alt_titles(alt_titles);
        }
        if let Some(description) = value.description {
            builder.description(description);
        }
        if let Some(authors) = value.authors {
            builder.authors(authors);
        }
        if let Some(artists) = value.artists {
            builder.artists(artists);
        }
        if let Some(links) = value.links {
            builder.links(links);
        }
        builder.original_language(value.original_language);
        if let Some(last_volume) = value.last_volume {
            builder.last_volume(last_volume);
        }
        if let Some(last_chapter) = value.last_chapter {
            builder.last_chapter(last_chapter);
        }
        if let Some(publication_demographic) = value.publication_demographic {
            builder.publication_demographic(publication_demographic);
        }
        builder.status(value.status);
        if let Some(year) = value.year {
            builder.year(year);
        }
        builder.content_rating(value.content_rating);
        if let Some(chapter_numbers_reset_on_new_volume) = value.chapter_numbers_reset_on_new_volume
        {
            builder.chapter_numbers_reset_on_new_volume(chapter_numbers_reset_on_new_volume);
        }
        if let Some(tags) = value.tags {
            builder.tags(tags);
        }
        if let Some(primary_cover) = value.primary_cover {
            builder.primary_cover(primary_cover);
        }
        builder.version(value.version);
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl UpdateMangaParam {
    pub async fn send(self, client: &MangaDexClient) -> Result<Limited<MangaData>> {
        <UpdateMangaBuilder as From<Self>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
