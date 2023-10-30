use crate::HttpClientRef;

pub mod get;

use get::MangaReadingStatusesBuilder;

#[derive(Debug)]
pub struct StatusEndpoint {
    http_client: HttpClientRef,
}

impl StatusEndpoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }
    pub fn get(&self) -> MangaReadingStatusesBuilder {
        MangaReadingStatusesBuilder::default().http_client(self.http_client.clone())
    }
}
