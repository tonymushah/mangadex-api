#[cfg(feature = "custom_list_v2")]
pub mod bookmark;
pub mod follow;
pub mod get;
pub mod list;

use crate::HttpClientRef;
#[cfg(feature = "custom_list_v2")]
use bookmark::BookmarkEndpoint;
use follow::FollowEndpoint;
use get::GetUserBuilder;
use list::ListEndpoint;
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

    pub fn get(&self) -> GetUserBuilder {
        GetUserBuilder::default()
            .user_id(self.id)
            .http_client(self.http_client.clone())
    }

    #[cfg_attr(
        feature = "custom_list_v2",
        deprecated(since = "3.0.0-alpha.1", note = "use .bookmark() instead")
    )]
    pub fn follow(&self) -> FollowEndpoint {
        FollowEndpoint::new(self.http_client.clone(), self.id)
    }

    #[cfg(feature = "custom_list_v2")]
    pub fn bookmark(&self) -> BookmarkEndpoint {
        BookmarkEndpoint::new(self.http_client.clone(), self.id)
    }

    pub fn list(&self) -> ListEndpoint {
        ListEndpoint::new(self.http_client.clone(), self.id)
    }
}
