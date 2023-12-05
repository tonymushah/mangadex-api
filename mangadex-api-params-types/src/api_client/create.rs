#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::api_client::post::CreateClientBuilder, MangaDexClient};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_schema::v5::ApiClientResponse;
use mangadex_api_types::ApiClientProfile;

#[derive(serde::Serialize, serde::Deserialize, specta::Type, Debug, Clone)]
pub struct ApiClientCreateParams {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub profile: ApiClientProfile,
    #[serde(default = "ApiClientCreateParams::default_version")]
    pub version: Option<u32>,
}

impl ApiClientCreateParams {
    fn default_version() -> Option<u32> {
        Some(1)
    }
    #[cfg(feature = "mangadex-api-resolver")]
    pub async fn send(self, client: &MangaDexClient) -> ApiClientResponse {
        let builder: CreateClientBuilder = self.into();
        builder
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<ApiClientCreateParams> for CreateClientBuilder {
    fn from(value: ApiClientCreateParams) -> Self {
        let mut buidler = Self::default();
        buidler.name(value.name);
        if let Some(description) = value.description {
            buidler.description(description);
        }
        buidler.profile(value.profile);
        if let Some(version) = value.version {
            buidler.version(version);
        }
        buidler
    }
}
