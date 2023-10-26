use crate::HttpClientRef;

pub mod get;

use get::ListTagsBuilder;

#[derive(Debug)]
pub struct TagEndpoint {
    http_client: HttpClientRef,
}

impl TagEndpoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }
    pub fn get(&self) -> ListTagsBuilder {
        ListTagsBuilder::default().http_client(self.http_client.clone())
    }
}
