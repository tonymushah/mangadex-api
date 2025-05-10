#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::Result;
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{
    v5::upload::upload_session_id::upload_session_file_id::delete::DeleteImageBuilder,
    MangaDexClient,
};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_schema::Limited;

use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
pub struct DeleteImageParam {
    pub session_id: Uuid,
    pub session_file_id: Uuid,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<DeleteImageParam> for DeleteImageBuilder {
    fn from(value: DeleteImageParam) -> Self {
        let mut builder = Self::default();
        builder.session_id(value.session_id);
        builder.session_file_id(value.session_file_id);
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl DeleteImageParam {
    pub async fn send(self, client: &MangaDexClient) -> Result<Limited<()>> {
        let data = <DeleteImageBuilder as From<Self>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await?;
        Ok(Limited {
            rate_limit: data.rate_limit,
            body: (),
        })
    }
}
