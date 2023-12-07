use crate::HttpClientRef;

use uuid::Uuid;

#[cfg(feature = "custom_list_v2")]
pub mod bookmark;
pub mod delete;
pub mod follow;
pub mod get;
pub mod put;

#[cfg(feature = "custom_list_v2")]
use bookmark::BookMarkEndpoint;
use delete::DeleteGroupBuilder;
use follow::FollowEndpoint;
use get::GetGroupBuilder;
use put::UpdateGroupBuilder;

#[derive(Debug)]
pub struct IdEndpoint {
    http_client: HttpClientRef,
    id: Uuid,
}

impl IdEndpoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef, id: Uuid) -> Self {
        Self { http_client, id }
    }
    #[cfg(feature = "custom_list_v2")]
    pub fn bookmark(&self) -> BookMarkEndpoint {
        BookMarkEndpoint::new(self.http_client.clone(), self.id)
    }
    #[cfg_attr(
        feature = "custom_list_v2",
        deprecated(since = "3.0.0-alpha.1", note = "use .bookmark() instead")
    )]
    pub fn follow(&self) -> FollowEndpoint {
        FollowEndpoint::new(self.http_client.clone(), self.id)
    }
    pub fn get(&self) -> GetGroupBuilder {
        GetGroupBuilder::default()
            .group_id(self.id)
            .http_client(self.http_client.clone())
    }
    pub fn delete(&self) -> DeleteGroupBuilder {
        DeleteGroupBuilder::default()
            .group_id(self.id)
            .http_client(self.http_client.clone())
    }
    pub fn put(&self) -> UpdateGroupBuilder {
        UpdateGroupBuilder::default()
            .group_id(self.id)
            .http_client(self.http_client.clone())
    }
}
