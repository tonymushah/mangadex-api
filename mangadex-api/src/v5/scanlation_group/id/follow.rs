use crate::HttpClientRef;

use uuid::Uuid;

pub mod delete;
pub mod post;

use delete::UnfollowGroupBuilder;
use post::FollowGroupBuilder;

#[deprecated(
    since = "3.0.0-alpha.1",
    note = "After the introduction of the Subscription system, this endpoint will be removed in 3.0.0. Please use the Bookmark Endpoint instead"
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
            .http_client(self.http_client.clone())
            .group_id(self.id)
    }
    pub fn delete(&self) -> UnfollowGroupBuilder {
        UnfollowGroupBuilder::default()
            .http_client(self.http_client.clone())
            .group_id(self.id)
    }
}
