#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::Result;
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::manga::random::get::GetRandomMangaBuilder, MangaDexClient};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_schema::{v5::MangaData, Limited};

use mangadex_api_types::{ContentRating, ReferenceExpansionResource, TagSearchMode};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Default, PartialEq, Eq)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(default)]
pub struct MangaRandomParams {
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub includes: Vec<ReferenceExpansionResource>,
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub content_rating: Vec<ContentRating>,
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub included_tags: Vec<Uuid>,
    pub included_tags_mode: Option<TagSearchMode>,
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub excluded_tags: Vec<Uuid>,
    pub excluded_tags_mode: Option<TagSearchMode>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<MangaRandomParams> for GetRandomMangaBuilder {
    fn from(value: MangaRandomParams) -> Self {
        let mut builder = Self::default();
        builder.includes(value.includes);
        builder.content_rating(value.content_rating);
        builder.included_tags(value.included_tags);
        if let Some(included_tags_mode) = value.included_tags_mode {
            builder.included_tags_mode(included_tags_mode);
        }
        builder.excluded_tags(value.excluded_tags);
        if let Some(excluded_tags_mode) = value.excluded_tags_mode {
            builder.excluded_tags_mode(excluded_tags_mode);
        }
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl MangaRandomParams {
    pub async fn send(self, client: &MangaDexClient) -> Result<Limited<MangaData>> {
        <GetRandomMangaBuilder as From<Self>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
