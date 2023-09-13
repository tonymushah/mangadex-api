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
            .http_client(self.http_client.clone())
            .chapter_id(self.id)
    }
    pub fn put(&self) -> UpdateChapterBuilder {
        UpdateChapterBuilder::default()
            .http_client(self.http_client.clone())
            .chapter_id(self.id)
    }
    pub fn delete(&self) -> DeleteChapterBuilder {
        DeleteChapterBuilder::default()
            .http_client(self.http_client.clone())
            .chapter_id(self.id)
    }
}
