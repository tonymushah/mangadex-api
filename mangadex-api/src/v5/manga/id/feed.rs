pub mod get;

use crate::HttpClientRef;

use uuid::Uuid;

use get::GetMangaFeedBuilder;

#[derive(Debug)]
pub struct FeedEndpoint {
    http_client: HttpClientRef,
    id: Uuid,
}

impl FeedEndpoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef, id: Uuid) -> Self {
        Self { http_client, id }
    }
    pub fn get(&self) -> GetMangaFeedBuilder {
        GetMangaFeedBuilder::default()
            .manga_id(self.id)
            .http_client(self.http_client.clone())
    }
}
