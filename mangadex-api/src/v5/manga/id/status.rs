pub mod get;
pub mod post;

use crate::HttpClientRef;

use uuid::Uuid;

use get::MangaReadingStatusBuilder;
use post::UpdateMangaReadingStatusBuilder;

#[derive(Debug)]
pub struct StatusEndpoint {
    http_client: HttpClientRef,
    id: Uuid,
}

impl StatusEndpoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef, id: Uuid) -> Self {
        Self { http_client, id }
    }
    pub fn get(&self) -> MangaReadingStatusBuilder {
        MangaReadingStatusBuilder::default()
            .http_client(self.http_client.clone())
            .manga_id(self.id)
    }
    pub fn post(&self) -> UpdateMangaReadingStatusBuilder {
        UpdateMangaReadingStatusBuilder::default()
            .http_client(self.http_client.clone())
            .manga_id(self.id)
    }
}
