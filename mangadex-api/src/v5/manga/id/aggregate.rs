pub mod get;

use crate::HttpClientRef;

use uuid::Uuid;

use get::GetMangaAggregateBuilder;

#[derive(Debug)]
pub struct AggregateEndpoint {
    http_client: HttpClientRef,
    id: Uuid,
}

impl AggregateEndpoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef, id: Uuid) -> Self {
        Self { http_client, id }
    }
    pub fn get(&self) -> GetMangaAggregateBuilder {
        GetMangaAggregateBuilder::default()
            .manga_id(self.id)
            .http_client(self.http_client.clone())
    }
}
