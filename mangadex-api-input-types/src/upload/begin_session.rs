#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::Result;
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::upload::begin::post::StartUploadSessionBuilder, MangaDexClient};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_schema::{v5::UploadSessionData, Limited};

use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct BeginUploadSessionParam {
    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
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
