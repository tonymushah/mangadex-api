#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::api_client::get::ListClientsBuilder, MangaDexClient};
use mangadex_api_types::{ApiClientState, ReferenceExpansionResource};

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ApiClientListParam {
    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub limit: Option<u32>,
    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub offset: Option<u32>,
    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub state: Option<ApiClientState>,
    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub name: Option<String>,
    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub includes: Vec<ReferenceExpansionResource>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl ApiClientListParam {
    pub async fn send(
        self,
        client: &MangaDexClient,
    ) -> mangadex_api_schema::v5::ApiClientListResponse {
        let builder: ListClientsBuilder = self.into();
        builder
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<ApiClientListParam> for ListClientsBuilder {
    fn from(value: ApiClientListParam) -> Self {
        let mut builder = Self::default();
        if let Some(limit) = value.limit {
            builder.limit(limit);
        }
        if let Some(offset) = value.offset {
            builder.offset(offset);
        }
        if let Some(state) = value.state {
            builder.state(state);
        }
        if let Some(name) = value.name {
            builder.name(name);
        }
        builder.includes(value.includes);
        builder
    }
}
