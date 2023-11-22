#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::user::follows::list::get::GetFollowedCustomListsBuilder, MangaDexClient};

use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Clone, Type, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UserFollowedListParams {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<UserFollowedListParams> for GetFollowedCustomListsBuilder {
    fn from(value: UserFollowedListParams) -> Self {
        let mut builder = Self::default();
        if let Some(limit) = value.limit {
            builder.limit(limit);
        }
        if let Some(offset) = value.offset {
            builder.offset(offset);
        }
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl UserFollowedListParams {
    pub async fn send(
        self,
        client: &MangaDexClient,
    ) -> mangadex_api_schema::v5::CustomListListResponse {
        <GetFollowedCustomListsBuilder as From<Self>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
