#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::user::follows::manga::get::FollowedMangaBuilder, MangaDexClient};

use mangadex_api_types::ReferenceExpansionResource;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Clone, Type, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UserFollowedMangaParams {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub includes: Vec<ReferenceExpansionResource>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<UserFollowedMangaParams> for FollowedMangaBuilder {
    fn from(value: UserFollowedMangaParams) -> Self {
        let mut builder = Self::default();
        if let Some(limit) = value.limit {
            builder.limit(limit);
        }
        if let Some(offset) = value.offset {
            builder.offset(offset);
        }
        builder.includes(value.includes);
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl UserFollowedMangaParams {
    pub async fn send(self, client: &MangaDexClient) -> mangadex_api_schema::v5::MangaListResponse {
        <FollowedMangaBuilder as From<Self>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
