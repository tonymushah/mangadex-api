#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{
    v5::upload::begin::chapter_id::post::StartEditChapterSessionBuilder, MangaDexClient,
};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_schema::{v5::UploadSessionData, Limited};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_types::error::Result;

use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
pub struct BeginEditUploadSessionParam {
    pub chapter_id: Uuid,

    pub version: u32,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<BeginEditUploadSessionParam> for StartEditChapterSessionBuilder {
    fn from(value: BeginEditUploadSessionParam) -> Self {
        let mut builder = Self::default();
        builder.chapter_id(value.chapter_id);
        builder.version(value.version);
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl BeginEditUploadSessionParam {
    pub async fn send(self, client: &MangaDexClient) -> Result<Limited<UploadSessionData>> {
        <StartEditChapterSessionBuilder as From<BeginEditUploadSessionParam>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
