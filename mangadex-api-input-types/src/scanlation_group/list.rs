#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::scanlation_group::get::ListGroupBuilder, MangaDexClient};

use mangadex_api_types::{GroupSortOrder, Language, ReferenceExpansionResource};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Default)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(default)]
pub struct ScanlationGroupListParams {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub group_ids: Vec<Uuid>,
    pub name: Option<String>,
    /// Language the scanlation primarily translates or uploads works into.
    // The corresponding response body field returns an array so this seems likely to change to accept an array of languages.
    pub focused_language: Option<Language>,
    #[cfg_attr(feature = "async-graphql", graphql(skip))]
    pub includes: Vec<ReferenceExpansionResource>,
    pub order: Option<GroupSortOrder>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<ScanlationGroupListParams> for ListGroupBuilder {
    fn from(value: ScanlationGroupListParams) -> Self {
        let mut builder = Self::default();
        if let Some(limit) = value.limit {
            builder.limit(limit);
        }
        if let Some(offset) = value.offset {
            builder.offset(offset);
        }
        builder.group_ids(value.group_ids);
        if let Some(name) = value.name {
            builder.name(name);
        }
        if let Some(focused_language) = value.focused_language {
            builder.focused_language(focused_language);
        }
        builder.includes(value.includes);
        if let Some(order) = value.order {
            builder.order(order);
        }
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl ScanlationGroupListParams {
    pub async fn send(self, client: &MangaDexClient) -> mangadex_api_schema::v5::GroupListResponse {
        <ListGroupBuilder as From<Self>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
