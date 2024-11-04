#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::api_client::id::delete::DeleteClientBuilder, MangaDexClient};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_types::error::Result;

use uuid::Uuid;

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
pub struct ApiClientDeleteParam {
    pub client_id: Uuid,
    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub version: Option<u32>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl ApiClientDeleteParam {
    pub async fn send(self, client: &MangaDexClient) -> Result<()> {
        let builder: DeleteClientBuilder = self.into();
        builder
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<ApiClientDeleteParam> for DeleteClientBuilder {
    fn from(value: ApiClientDeleteParam) -> Self {
        let mut builder = Self::default();
        builder.client_id(value.client_id);
        builder.version(value.version);
        builder
    }
}
