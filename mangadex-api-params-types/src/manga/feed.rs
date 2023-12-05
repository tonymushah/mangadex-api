#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::manga::id::feed::get::GetMangaFeedBuilder, MangaDexClient};

use mangadex_api_types::{
    ContentRating, IncludeExternalUrl, IncludeFuturePages, IncludeFuturePublishAt,
    IncludeFutureUpdates, Language, MangaDexDateTime, MangaFeedSortOrder,
    ReferenceExpansionResource,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct MangaFeedParams {
    pub manga_id: Uuid,

    // `manga_id` cannot use the `Default` trait so these attributes have to be manually set.
    #[serde(default)]
    pub limit: Option<u32>,
    #[serde(default)]
    pub offset: Option<u32>,
    #[serde(default)]
    pub translated_language: Vec<Language>,
    #[serde(default)]
    pub original_language: Vec<Language>,
    #[serde(default)]
    pub excluded_original_language: Vec<Language>,
    #[serde(default)]
    pub content_rating: Vec<ContentRating>,
    /// Groups to exclude from the results.
    #[serde(default)]
    pub excluded_groups: Vec<Uuid>,
    /// Uploaders to exclude from the results.
    #[serde(default)]
    pub excluded_uploaders: Vec<Uuid>,
    /// Flag to include future chapter updates in the results.
    ///
    /// Default: `IncludeFutureUpdates::Include` (1)
    #[serde(default)]
    pub include_future_updates: Option<IncludeFutureUpdates>,
    /// DateTime string with following format: `YYYY-MM-DDTHH:MM:SS`.
    #[serde(default)]
    pub created_at_since: Option<MangaDexDateTime>,
    /// DateTime string with following format: `YYYY-MM-DDTHH:MM:SS`.
    #[serde(default)]
    pub updated_at_since: Option<MangaDexDateTime>,
    /// DateTime string with following format: `YYYY-MM-DDTHH:MM:SS`.
    #[serde(default)]
    pub publish_at_since: Option<MangaDexDateTime>,
    #[serde(default)]
    pub order: Option<MangaFeedSortOrder>,
    #[serde(default)]
    pub includes: Vec<ReferenceExpansionResource>,
    #[serde(default)]
    pub include_empty_pages: Option<IncludeFuturePages>,
    #[serde(default)]
    pub include_future_publish_at: Option<IncludeFuturePublishAt>,
    #[serde(default)]
    pub include_external_url: Option<IncludeExternalUrl>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<MangaFeedParams> for GetMangaFeedBuilder {
    fn from(value: MangaFeedParams) -> Self {
        let mut builder = Self::default();
        builder.manga_id(value.manga_id);
        if let Some(limit) = value.limit {
            builder.limit(limit);
        }
        if let Some(offset) = value.offset {
            builder.offset(offset);
        }
        builder.translated_language(value.translated_language);
        builder.original_language(value.original_language);
        builder.excluded_original_language(value.excluded_original_language);
        builder.content_rating(value.content_rating);
        builder.excluded_groups(value.excluded_groups);
        builder.excluded_uploaders(value.excluded_uploaders);
        if let Some(include_future_updates) = value.include_future_updates {
            builder.include_future_updates(include_future_updates);
        }
        if let Some(include_empty_pages) = value.include_empty_pages {
            builder.include_empty_pages(include_empty_pages);
        }
        if let Some(include_future_publish_at) = value.include_future_publish_at {
            builder.include_future_publish_at(include_future_publish_at);
        }
        if let Some(include_external_url) = value.include_external_url {
            builder.include_external_url(include_external_url);
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
        if let Some(order) = value.order {
            builder.order(order);
        }
        builder.includes(value.includes);
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl MangaFeedParams {
    pub async fn send(
        self,
        client: &MangaDexClient,
    ) -> mangadex_api_schema::v5::ChapterListResponse {
        <GetMangaFeedBuilder as From<Self>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
