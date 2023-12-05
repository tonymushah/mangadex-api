#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::user::follows::user::get::FollowedUsersBuilder, MangaDexClient};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(default)]
pub struct UserFollowedUserParams {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<UserFollowedUserParams> for FollowedUsersBuilder {
    fn from(value: UserFollowedUserParams) -> Self {
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
impl UserFollowedUserParams {
    pub async fn send(self, client: &MangaDexClient) -> mangadex_api_schema::v5::UserListResponse {
        <FollowedUsersBuilder as From<Self>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
