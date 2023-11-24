#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::user::get::ListUserBuilder, MangaDexClient};

use mangadex_api_types::UserSortOrder;
use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Debug, Clone, Type, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UserListParam {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub user_ids: Vec<Uuid>,
    pub username: Option<String>,
    pub order: Option<UserSortOrder>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<UserListParam> for ListUserBuilder {
    fn from(value: UserListParam) -> Self {
        let mut builder = Self::default();
        if let Some(limit) = value.limit {
            builder.limit(limit);
        }
        if let Some(offset) = value.offset {
            builder.offset(offset);
        }
        builder.user_ids(value.user_ids);
        if let Some(username) = value.username {
            builder.username(username);
        }
        if let Some(order) = value.order {
            builder.order(order);
        }
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl UserListParam {
    pub async fn send(self, client: &MangaDexClient) -> mangadex_api_schema::v5::UserListResponse {
        <ListUserBuilder as From<Self>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
