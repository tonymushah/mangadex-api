#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::api_client::id::get::GetClientBuilder, MangaDexClient};
use mangadex_api_types::ReferenceExpansionResource;
use uuid::Uuid;

#[derive(serde::Deserialize, Debug, Clone)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct ApiClientGetUniqueParams {
    pub client_id: Uuid,

    #[serde(default)]
    pub includes: Vec<ReferenceExpansionResource>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl ApiClientGetUniqueParams {
    pub async fn send(self, client: &MangaDexClient) -> mangadex_api_schema::v5::ApiClientResponse {
        let builder: GetClientBuilder = self.into();
        builder
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<ApiClientGetUniqueParams> for GetClientBuilder {
    fn from(value: ApiClientGetUniqueParams) -> Self {
        let mut builder = Self::default();
        builder.client_id(value.client_id);
        builder.includes(value.includes);
        builder
    }
}
