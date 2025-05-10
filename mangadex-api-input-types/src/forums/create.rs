#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{rate_limit::Limited, Result};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::forums::thread::post::CreateForumThreadBuilder, MangaDexClient};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_schema::v5::ForumThreadResponseData;

use mangadex_api_types::ForumThreadType;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
pub struct CreateForumThreadParams {
    pub type_: ForumThreadType,
    pub id: Uuid,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<CreateForumThreadParams> for CreateForumThreadBuilder {
    fn from(value: CreateForumThreadParams) -> Self {
        let mut builder = Self::default();
        builder.type_(value.type_);
        builder.id(value.id);
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl CreateForumThreadParams {
    pub async fn send(self, client: &MangaDexClient) -> Result<Limited<ForumThreadResponseData>> {
        <CreateForumThreadBuilder as From<Self>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
