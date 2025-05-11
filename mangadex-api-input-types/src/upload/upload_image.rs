use crate::PathBuf;

#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{rate_limit::Limited, Result};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{
    v5::upload::upload_session_id::post::{UploadImage, UploadImagesBuilder},
    MangaDexClient,
};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_schema::v5::UploadSessionFileDataObject;

use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct UploadImageParam {
    pub session_id: Uuid,
    pub files: Vec<PathBuf>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<UploadImageParam> for UploadImagesBuilder {
    fn from(value: UploadImageParam) -> Self {
        let mut builder = Self::default();
        builder.session_id(value.session_id);
        let files: Vec<UploadImage> = value
            .files
            .iter()
            .map(|p| <std::path::PathBuf as From<PathBuf>>::from(p.clone()))
            .flat_map(<UploadImage as TryFrom<std::path::PathBuf>>::try_from)
            .collect();
        builder.files(files);
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl UploadImageParam {
    pub async fn send(
        self,
        client: &MangaDexClient,
    ) -> Result<Limited<UploadSessionFileDataObject>> {
        <UploadImagesBuilder as From<Self>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
