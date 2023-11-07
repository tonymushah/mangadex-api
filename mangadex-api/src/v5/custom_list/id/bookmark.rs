use uuid::Uuid;

use crate::HttpClientRef;

pub mod delete;
pub mod post;

use delete::UnBookMarkCustomListBuilder;
use post::BookMarkCustomListBuilder;

#[derive(Debug, Clone)]
pub struct BookMarkEndpoint {
    http_client: HttpClientRef,
    id: Uuid,
}

impl BookMarkEndpoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef, id: Uuid) -> Self {
        Self { http_client, id }
    }
    pub fn post(&self) -> BookMarkCustomListBuilder {
        BookMarkCustomListBuilder::default()
            .http_client(self.http_client.clone())
            .list_id(self.id)
    }
    pub fn delete(&self) -> UnBookMarkCustomListBuilder {
        UnBookMarkCustomListBuilder::default()
            .list_id(self.id)
            .http_client(self.http_client.clone())
    }
}
