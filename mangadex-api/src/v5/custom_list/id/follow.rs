use uuid::Uuid;

use crate::HttpClientRef;

pub mod delete;
pub mod post;

use delete::UnFollowCustomListBuilder;
use post::FollowCustomListBuilder;

#[derive(Debug, Clone)]
pub struct FollowEndpoint {
    http_client: HttpClientRef,
    id: Uuid,
}

impl FollowEndpoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef, id: Uuid) -> Self {
        Self { http_client, id }
    }
    pub fn post(&self) -> FollowCustomListBuilder {
        FollowCustomListBuilder::default()
            .list_id(self.id)
            .http_client(self.http_client.clone())
    }
    pub fn delete(&self) -> UnFollowCustomListBuilder {
        UnFollowCustomListBuilder::default()
            .list_id(self.id)
            .http_client(self.http_client.clone())
    }
}
