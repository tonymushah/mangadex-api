use crate::HttpClientRef;

pub mod post;

use post::LegacyIdMappingBuilder;

#[derive(Debug, Clone)]
pub struct MappingEndpoint {
    http_client: HttpClientRef,
}

impl MappingEndpoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }
    pub fn post(&self) -> LegacyIdMappingBuilder {
        LegacyIdMappingBuilder::default()
            .http_client(self.http_client.clone())
    }
}
