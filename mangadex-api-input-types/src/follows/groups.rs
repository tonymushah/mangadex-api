#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::user::follows::group::get::FollowedGroupsBuilder, MangaDexClient};

use mangadex_api_types::ReferenceExpansionResource;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default, PartialEq, Eq)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(default)]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct UserFollowedGroupsParams {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    #[cfg_attr(feature = "async-graphql", graphql(skip))]
    pub includes: Vec<ReferenceExpansionResource>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<UserFollowedGroupsParams> for FollowedGroupsBuilder {
    fn from(value: UserFollowedGroupsParams) -> Self {
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
impl UserFollowedGroupsParams {
    pub async fn send(
        self,
        client: &MangaDexClient,
    ) -> mangadex_api::Result<mangadex_api_schema::v5::GroupCollection> {
        <FollowedGroupsBuilder as From<Self>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
