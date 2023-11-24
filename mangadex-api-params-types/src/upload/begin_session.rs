#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::upload::begin::post::StartUploadSessionBuilder, MangaDexClient};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_schema::{v5::UploadSessionData, Limited};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_types::error::Result;

use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
pub struct BeginUploadSessionParam {
    #[serde(default)]
    pub groups: Vec<Uuid>,
    #[serde(rename = "manga")]
    pub manga_id: Uuid,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<BeginUploadSessionParam> for StartUploadSessionBuilder {
    fn from(value: BeginUploadSessionParam) -> Self {
        let mut builder = Self::default();
        builder.groups(value.groups);
        builder.manga_id(value.manga_id);
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl BeginUploadSessionParam {
    pub async fn send(self, client: &MangaDexClient) -> Result<Limited<UploadSessionData>> {
        <StartUploadSessionBuilder as From<Self>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
