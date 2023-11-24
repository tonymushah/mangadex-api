use std::collections::HashMap;

#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::settings::post::CreateOrUpdateUserSettingsBuilder, MangaDexClient};

use mangadex_api_types::MangaDexDateTime;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
pub struct CreateOrUpdateUserSettingsParams {
    #[serde(default)]
    pub settings: HashMap<String, String>,
    #[serde(default)]
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
