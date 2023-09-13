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
        todo!("implement the post method please")
    }
    pub fn delete(&self) -> UnBookMarkCustomListBuilder {
        todo!("implement the delete method please")
    }
}
