use uuid::Uuid;

pub mod delete;
pub mod post;

use crate::HttpClientRef;
use delete::DeleteMangaBatchViaCustomListBuilder;
use post::AddMangaBatchViaCustomListBuilder;

#[derive(Debug, Clone)]
pub struct BatchMangaEndpoint {
    http_client: HttpClientRef,
    id: Uuid,
}

impl BatchMangaEndpoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef, id: Uuid) -> Self {
        Self { http_client, id }
    }
    pub fn post(&self) -> AddMangaBatchViaCustomListBuilder {
        AddMangaBatchViaCustomListBuilder::default()
            .http_client(self.http_client.clone())
            .list_id(self.id)
    }
    pub fn delete(&self) -> DeleteMangaBatchViaCustomListBuilder {
        DeleteMangaBatchViaCustomListBuilder::default()
            .http_client(self.http_client.clone())
            .list_id(self.id)
    }
}
