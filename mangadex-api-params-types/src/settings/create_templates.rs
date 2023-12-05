#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::settings::template::post::CreateSettingsTemplateBuilder, MangaDexClient};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct CreateSettingsTemplateParams {
    pub description: Option<String>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<CreateSettingsTemplateParams> for CreateSettingsTemplateBuilder {
    fn from(value: CreateSettingsTemplateParams) -> Self {
        let mut builder = Self::default();
        if let Some(description) = value.description {
            builder.description(description);
        }
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl CreateSettingsTemplateParams {
    pub async fn send(self, client: &MangaDexClient) -> mangadex_api_types::error::Result<()> {
        <CreateSettingsTemplateBuilder as From<Self>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await?;
        Ok(())
    }
}
