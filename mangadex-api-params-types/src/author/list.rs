#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::author::get::ListAuthorBuilder, MangaDexClient};

use mangadex_api_types::{AuthorSortOrder, ReferenceExpansionResource};
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
pub struct AuthorListParams {
    #[serde(default)]
    pub limit: Option<u32>,
    #[serde(default)]
    pub offset: Option<u32>,
    #[serde(default)]
    pub author_ids: Vec<Uuid>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub order: Option<AuthorSortOrder>,
    #[serde(default)]
    pub includes: Vec<ReferenceExpansionResource>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl AuthorListParams {
    pub async fn send(
        self,
        client: &MangaDexClient,
    ) -> mangadex_api_schema::v5::AuthorListResponse {
        let builder: ListAuthorBuilder = self.into();
        builder
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<AuthorListParams> for ListAuthorBuilder {
    fn from(value: AuthorListParams) -> Self {
        let mut builder = Self::default();
        if let Some(limit) = value.limit {
            builder.limit(limit);
        }
        if let Some(offset) = value.offset {
            builder.offset(offset);
        }
        builder.author_ids(value.author_ids);
        if let Some(name) = value.name {
            builder.name(name);
        }
        if let Some(order) = value.order {
            builder.order(order);
        }
        builder.includes(value.includes);
        builder
    }
}
