#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::settings::template::post::CreateSettingsTemplateBuilder, MangaDexClient};

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
pub struct CreateSettingsTemplateParams {
    pub template: Option<String>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<CreateSettingsTemplateParams> for CreateSettingsTemplateBuilder {
    fn from(value: CreateSettingsTemplateParams) -> Self {
        let mut builder = Self::default();
        if let Some(template) = value.template {
            builder.template(template);
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
