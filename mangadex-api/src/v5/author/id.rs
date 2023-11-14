pub mod delete;
pub mod get;
pub mod put;

use uuid::Uuid;

use crate::HttpClientRef;

use delete::DeleteAuthorBuilder;
use get::GetAuthorBuilder;
use put::UpdateAuthorBuilder;

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

    pub fn get(&self) -> GetAuthorBuilder {
        GetAuthorBuilder::default()
            .author_id(self.id)
            .http_client(self.http_client.clone())
    }
    pub fn delete(&self) -> DeleteAuthorBuilder {
        DeleteAuthorBuilder::default()
            .author_id(self.id)
            .http_client(self.http_client.clone())
    }
    pub fn put(&self) -> UpdateAuthorBuilder {
        UpdateAuthorBuilder::default()
            .author_id(self.id)
            .http_client(self.http_client.clone())
    }
}
