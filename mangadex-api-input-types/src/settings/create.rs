use std::collections::HashMap;

#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::settings::post::CreateOrUpdateUserSettingsBuilder, MangaDexClient};

use mangadex_api_types::MangaDexDateTime;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
pub struct CreateOrUpdateUserSettingsParams {
    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub settings: HashMap<String, String>,
    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub updated_at: MangaDexDateTime,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<CreateOrUpdateUserSettingsParams> for CreateOrUpdateUserSettingsBuilder {
    fn from(value: CreateOrUpdateUserSettingsParams) -> Self {
        let mut builder = Self::default();
        builder.settings(value.settings);
        builder.updated_at(value.updated_at);
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl CreateOrUpdateUserSettingsParams {
    pub async fn send(
        self,
        client: &MangaDexClient,
    ) -> mangadex_api_schema::v5::UserSettingsResponse {
        <CreateOrUpdateUserSettingsBuilder as From<Self>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
