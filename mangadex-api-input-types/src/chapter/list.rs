#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::chapter::get::ListChapterBuilder, MangaDexClient};

use mangadex_api_types::{
    ChapterSortOrder, ContentRating, IncludeExternalUrl, IncludeFuturePages,
    IncludeFuturePublishAt, IncludeFutureUpdates, Language, MangaDexDateTime,
    ReferenceExpansionResource,
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ChapterListParams {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub chapter_ids: Vec<Uuid>,
    pub title: Option<String>,
    pub groups: Vec<Uuid>,
    pub uploaders: Vec<Uuid>,
    pub manga_id: Option<Uuid>,
    pub volumes: Vec<String>,
    /// Chapter number in the series or volume.
    pub chapters: Vec<String>,
    pub translated_languages: Vec<Language>,
    pub original_languages: Vec<Language>,
    pub excluded_original_languages: Vec<Language>,
    pub content_rating: Vec<ContentRating>,
    /// Groups to exclude from the results.
    pub excluded_groups: Vec<Uuid>,
    /// Uploaders to exclude from the results.
    pub excluded_uploaders: Vec<Uuid>,
    /// Flag to include future chapter updates in the results.
    ///
    /// Default: `IncludeFutureUpdates::Include` (1)
    pub include_future_updates: Option<IncludeFutureUpdates>,
    /// DateTime string with following format: `YYYY-MM-DDTHH:MM:SS`.
    pub created_at_since: Option<MangaDexDateTime>,
    /// DateTime string with following format: `YYYY-MM-DDTHH:MM:SS`.
    pub updated_at_since: Option<MangaDexDateTime>,
    /// DateTime string with following format: `YYYY-MM-DDTHH:MM:SS`.
    pub publish_at_since: Option<MangaDexDateTime>,
    /// Include empty pages
    pub include_empty_pages: Option<IncludeFuturePages>,
    /// Include external url chapters
    pub include_external_url: Option<IncludeExternalUrl>,
    /// Include future publish at
    pub include_future_publish_at: Option<IncludeFuturePublishAt>,
    pub order: Option<ChapterSortOrder>,
    pub includes: Vec<ReferenceExpansionResource>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<ChapterListParams> for ListChapterBuilder {
    fn from(value: ChapterListParams) -> Self {
        let mut builder = Self::default();
        if let Some(limit) = value.limit {
            builder.limit(limit);
        }
        if let Some(offset) = value.offset {
            builder.offset(offset);
        }
        builder.chapter_ids(value.chapter_ids);
        if let Some(title) = value.title {
            builder.title(title);
        }
        builder.groups(value.groups);
        builder.uploaders(value.uploaders);
        if let Some(manga_id) = value.manga_id {
            builder.manga_id(manga_id);
        }
        builder.volumes(value.volumes);
        builder.chapters(value.chapters);
        builder.translated_languages(value.translated_languages);
        builder.original_languages(value.original_languages);
        builder.excluded_original_languages(value.excluded_original_languages);
        builder.content_rating(value.content_rating);
        builder.excluded_groups(value.excluded_groups);
        builder.excluded_uploaders(value.excluded_uploaders);
        if let Some(include_future_updates) = value.include_future_updates {
            builder.include_future_updates(include_future_updates);
        }
        if let Some(created_at_since) = value.created_at_since {
            builder.created_at_since(created_at_since);
        }
        if let Some(updated_at_since) = value.updated_at_since {
            builder.updated_at_since(updated_at_since);
        }
        if let Some(publish_at_since) = value.publish_at_since {
            builder.publish_at_since(publish_at_since);
        }
        if let Some(include_empty_pages) = value.include_empty_pages {
            builder.include_empty_pages(include_empty_pages);
        }
        if let Some(include_external_url) = value.include_external_url {
            builder.include_external_url(include_external_url);
        }
        if let Some(include_future_publish_at) = value.include_future_publish_at {
            builder.include_future_publish_at(include_future_publish_at);
        }
        if let Some(order) = value.order {
            builder.order(order);
        }
        builder.includes(value.includes);
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl ChapterListParams {
    pub async fn send(
        self,
        client: &MangaDexClient,
    ) -> mangadex_api_schema::v5::ChapterListResponse {
        let builder: ListChapterBuilder = self.into();
        builder
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
