#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{
    v5::user::follows::manga::feed::get::GetFollowedMangaFeedBuilder, MangaDexClient,
};

use mangadex_api_types::{
    ContentRating, IncludeExternalUrl, IncludeFuturePages, IncludeFuturePublishAt,
    IncludeFutureUpdates, Language, MangaDexDateTime, MangaFeedSortOrder,
    ReferenceExpansionResource,
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
#[serde(default)]
pub struct FollowedMangaFeedParams {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub translated_language: Vec<Language>,
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub original_language: Vec<Language>,
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub excluded_original_language: Vec<Language>,
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub content_rating: Vec<ContentRating>,
    /// Groups to exclude from the results.
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub excluded_groups: Vec<Uuid>,
    /// Uploaders to exclude from the results.
    #[cfg_attr(feature = "async-graphql", graphql(default))]
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
    pub order: Option<MangaFeedSortOrder>,
    #[cfg_attr(feature = "async-graphql", graphql(skip))]
    pub includes: Vec<ReferenceExpansionResource>,
    pub include_empty_pages: Option<IncludeFuturePages>,
    pub include_future_publish_at: Option<IncludeFuturePublishAt>,
    pub include_external_url: Option<IncludeExternalUrl>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<FollowedMangaFeedParams> for GetFollowedMangaFeedBuilder {
    fn from(value: FollowedMangaFeedParams) -> Self {
        let mut builder = Self::default();
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
impl FollowedMangaFeedParams {
    pub async fn send(
        self,
        client: &MangaDexClient,
    ) -> mangadex_api_schema::v5::ChapterListResponse {
        <GetFollowedMangaFeedBuilder as From<Self>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
