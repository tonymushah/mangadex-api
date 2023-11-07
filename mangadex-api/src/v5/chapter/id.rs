pub mod delete;
pub mod get;
pub mod put;

use delete::DeleteChapterBuilder;
use get::GetChapterBuilder;
use put::UpdateChapterBuilder;

use crate::HttpClientRef;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct IdEndpoint {
    http_client: HttpClientRef,
    id: Uuid,
}

impl IdEndpoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef, id: Uuid) -> Self {
        Self { http_client, id }
    }
    pub fn get(&self) -> GetChapterBuilder {
        GetChapterBuilder::default()
            .chapter_id(self.id)
            .http_client(self.http_client.clone())
    }
    pub fn put(&self) -> UpdateChapterBuilder {
        UpdateChapterBuilder::default()
            .chapter_id(self.id)
            .http_client(self.http_client.clone())
    }
    pub fn delete(&self) -> DeleteChapterBuilder {
        DeleteChapterBuilder::default()
            .chapter_id(self.id)
            .http_client(self.http_client.clone())
    }
}
