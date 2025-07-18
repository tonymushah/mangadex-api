#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{rate_limit::Limited, Result};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{
    v5::upload::upload_session_id::batch::delete::DeleteImagesBuilder, MangaDexClient,
};

use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct DeleteImagesParam {
    pub session_id: Uuid,
    pub session_file_ids: Vec<Uuid>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<DeleteImagesParam> for DeleteImagesBuilder {
    fn from(value: DeleteImagesParam) -> Self {
        let mut builder = Self::default();
        builder.session_id(value.session_id);
        builder.session_file_ids(value.session_file_ids);
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl DeleteImagesParam {
    pub async fn send(self, client: &MangaDexClient) -> Result<Limited<()>> {
        let res = <DeleteImagesBuilder as From<DeleteImagesParam>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await?;
        Ok(res.drop_body())
    }
}
