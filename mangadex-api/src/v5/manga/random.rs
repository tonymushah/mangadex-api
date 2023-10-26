use crate::HttpClientRef;

pub mod get;

use get::GetRandomMangaBuilder;

#[derive(Debug)]
pub struct RandomEndpoint {
    http_client: HttpClientRef,
}

impl RandomEndpoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }
    pub fn get(&self) -> GetRandomMangaBuilder {
        GetRandomMangaBuilder::default().http_client(self.http_client.clone())
    }
}
