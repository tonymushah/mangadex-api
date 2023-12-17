#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::api_client::id::post::EditClientBuilder, MangaDexClient};

use uuid::Uuid;

#[derive(serde::Deserialize, Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
pub struct ApiClientEditParam {
    pub client_id: Uuid,
    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub description: Option<String>,
    // >= 1
    pub version: u32,
}

#[cfg(feature = "mangadex-api-resolver")]
impl ApiClientEditParam {
    pub async fn send(self, client: &MangaDexClient) -> mangadex_api_schema::v5::ApiClientResponse {
        let builder: EditClientBuilder = self.into();
        builder
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<ApiClientEditParam> for EditClientBuilder {
    fn from(value: ApiClientEditParam) -> Self {
        let mut builder = Self::default();
        builder.client_id(value.client_id);
        if let Some(description) = value.description {
            builder.description(description);
        }
        builder.version(value.version);
        builder
    }
}
