use crate::HttpClientRef;

use uuid::Uuid;

pub mod bookmark;
pub mod delete;
pub mod follow;
pub mod get;
pub mod put;

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
    pub fn bookmark(&self) -> BookMarkEndpoint {
        BookMarkEndpoint::new(self.http_client.clone(), self.id)
    }
    #[deprecated(since = "3.0.0-rc1", note = "use .bookmark() instead")]
    pub fn follow(&self) -> FollowEndpoint {
        FollowEndpoint::new(self.http_client.clone(), self.id)
    }
    pub fn get(&self) -> GetGroupBuilder {
        GetGroupBuilder::default()
            .http_client(self.http_client.clone())
            .group_id(self.id)
    }
    pub fn delete(&self) -> DeleteGroupBuilder{
        DeleteGroupBuilder::default()
            .http_client(self.http_client.clone())
            .group_id(self.id)
    }
    pub fn put(&self) -> UpdateGroupBuilder{
        UpdateGroupBuilder::default()
            .http_client(self.http_client.clone())
            .group_id(self.id)
    }
}
