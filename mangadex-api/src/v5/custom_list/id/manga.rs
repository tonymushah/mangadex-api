use crate::HttpClientRef;
use uuid::Uuid;
pub mod get;

#[derive(Debug, Clone)]
pub struct MangaEndpoint {
    http_client: HttpClientRef,
    id: Uuid,
}

impl MangaEndpoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef, id: Uuid) -> Self {
        Self { http_client, id }
    }
    pub fn get(&self) {
        todo!("Implement the get method please")
    }
}
