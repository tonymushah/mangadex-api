pub mod bookmark;
#[cfg(feature = "legacy-account")]
pub mod delete;
pub mod follow;
pub mod get;
pub mod list;

use bookmark::BookmarkEndpoint;
#[cfg(feature = "legacy-account")]
use delete::DeleteUserBuilder;
use follow::FollowEndpoint;
use get::GetUserBuilder;
use list::ListEndpoint;

use crate::HttpClientRef;
use uuid::Uuid;

#[derive(Debug)]
pub struct IdEndpoint {
    http_client: HttpClientRef,
    id: Uuid,
}

impl IdEndpoint {
    pub fn new(http_client: HttpClientRef, id: Uuid) -> Self {
        Self { http_client, id }
    }

    #[cfg(feature = "legacy-account")]
    pub fn delete(&self) -> DeleteUserBuilder {
        DeleteUserBuilder::default()
            .user_id(self.id)
            .http_client(self.http_client.clone())
    }

    pub fn get(&self) -> GetUserBuilder {
        GetUserBuilder::default()
            .user_id(self.id)
            .http_client(self.http_client.clone())
    }

    #[deprecated(since = "3.0.0-alpha.1", note = "use .bookmark() instead")]
    pub fn follow(&self) -> FollowEndpoint {
        FollowEndpoint::new(self.http_client.clone(), self.id)
    }

    pub fn bookmark(&self) -> BookmarkEndpoint {
        BookmarkEndpoint::new(self.http_client.clone(), self.id)
    }

    pub fn list(&self) -> ListEndpoint {
        ListEndpoint::new(self.http_client.clone(), self.id)
    }
}
