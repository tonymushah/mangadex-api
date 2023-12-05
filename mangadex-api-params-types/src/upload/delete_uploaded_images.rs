#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{
    v5::upload::upload_session_id::batch::delete::DeleteImagesBuilder, MangaDexClient,
};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_schema::Limited;
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_types::error::Result;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
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
        Ok(Limited {
            rate_limit: res.rate_limit,
            body: (),
        })
    }
}
