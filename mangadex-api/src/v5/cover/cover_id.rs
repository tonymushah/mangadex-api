pub mod delete;
pub mod get;
pub mod put;

use delete::DeleteCoverBuilder;
use get::GetCoverBuilder;
use put::EditCoverBuilder;
use uuid::Uuid;

use crate::HttpClientRef;

#[derive(Debug, Clone)]
pub struct CoverIdEndpoint {
    http_client: HttpClientRef,
    cover_id: Uuid,
}

impl CoverIdEndpoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef, cover_id: Uuid) -> Self {
        Self {
            http_client,
            cover_id,
        }
    }
    pub fn delete(&self) -> DeleteCoverBuilder {
        DeleteCoverBuilder::default()
            .http_client(self.http_client.clone())
            .cover_id(self.cover_id)
    }
    pub fn get(&self) -> GetCoverBuilder {
        GetCoverBuilder::default()
            .http_client(self.http_client.clone())
            .cover_id(self.cover_id)
    }
    pub fn put(&self) -> EditCoverBuilder {
        EditCoverBuilder::default()
            .http_client(self.http_client.clone())
            .cover_id(self.cover_id)
    }
}
