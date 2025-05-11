use crate::HttpClientRef;

use uuid::Uuid;

pub mod delete;
pub mod post;

use delete::UnfollowGroupBuilder;
use post::FollowGroupBuilder;

#[cfg_attr(
    feature = "custom_list_v2",
    deprecated(
        since = "3.0.0-rc.1",
        note = "After the introduction of the Subscription system, this endpoint will be removed in a major version."
    )
)]
#[derive(Debug)]
pub struct FollowEndpoint {
    http_client: HttpClientRef,
    id: Uuid,
}

impl FollowEndpoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef, id: Uuid) -> Self {
        Self { http_client, id }
    }
    pub fn post(&self) -> FollowGroupBuilder {
        FollowGroupBuilder::default()
            .group_id(self.id)
            .http_client(self.http_client.clone())
    }
    pub fn delete(&self) -> UnfollowGroupBuilder {
        UnfollowGroupBuilder::default()
            .group_id(self.id)
            .http_client(self.http_client.clone())
    }
}
