use uuid::Uuid;

use crate::HttpClientRef;

pub mod get;

use get::CustomListMangaFeedBuilder;

#[derive(Debug, Clone)]
pub struct FeedEndPoint {
    http_client: HttpClientRef,
    id: Uuid,
}

impl FeedEndPoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef, id: Uuid) -> Self {
        Self { http_client, id }
    }
    pub fn get(&self) -> CustomListMangaFeedBuilder {
        CustomListMangaFeedBuilder::default()
            .http_client(self.http_client.clone())
            .list_id(self.id)
    }
}
