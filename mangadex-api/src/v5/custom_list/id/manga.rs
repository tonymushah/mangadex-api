use crate::HttpClientRef;
use uuid::Uuid;
pub mod get;

use get::GetCustomListMangaBuilder;

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
    pub fn get(&self) -> GetCustomListMangaBuilder {
        GetCustomListMangaBuilder::default()
            .http_client(self.http_client.clone())
            .list_id(self.id)
    }
}
