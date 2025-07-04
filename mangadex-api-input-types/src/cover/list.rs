#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::cover::get::ListCoverBuilder, MangaDexClient};

use mangadex_api_types::{CoverSortOrder, Language, ReferenceExpansionResource};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Default, PartialEq, Eq)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
#[serde(default)]
pub struct CoverListParam {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub manga_ids: Vec<Uuid>,
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub cover_ids: Vec<Uuid>,
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub uploader_ids: Vec<Uuid>,
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub locales: Vec<Language>,
    pub order: Option<CoverSortOrder>,
    #[cfg_attr(feature = "async-graphql", graphql(skip))]
    pub includes: Vec<ReferenceExpansionResource>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<CoverListParam> for ListCoverBuilder {
    fn from(value: CoverListParam) -> Self {
        let mut builder = Self::default();
        if let Some(limit) = value.limit {
            builder.limit(limit);
        }
        if let Some(offset) = value.offset {
            builder.offset(offset);
        }
        builder.manga_ids(value.manga_ids);
        builder.cover_ids(value.cover_ids);
        builder.locales(value.locales);
        if let Some(order) = value.order {
            builder.order(order);
        }
        builder.includes(value.includes);
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl CoverListParam {
    pub async fn send(
        self,
        client: &MangaDexClient,
    ) -> mangadex_api::Result<mangadex_api_schema::v5::CoverCollection> {
        let builder: ListCoverBuilder = self.into();
        builder
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
