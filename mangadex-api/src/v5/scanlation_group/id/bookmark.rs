use crate::HttpClientRef;

use uuid::Uuid;

pub mod delete;
pub mod post;

use delete::UnBookMarkGroupBuilder;
use post::BookmarkGroupBuilder;

#[derive(Debug)]
pub struct BookMarkEndpoint {
    http_client: HttpClientRef,
    id: Uuid,
}

impl BookMarkEndpoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef, id: Uuid) -> Self {
        Self { http_client, id }
    }
    pub fn post(&self) -> BookmarkGroupBuilder {
        BookmarkGroupBuilder::default()
            .http_client(self.http_client.clone())
            .group_id(self.id)
    }
    pub fn delete(&self) -> UnBookMarkGroupBuilder {
        UnBookMarkGroupBuilder::default()
            .http_client(self.http_client.clone())
            .group_id(self.id)
    }
}
