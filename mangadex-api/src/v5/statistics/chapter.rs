use crate::HttpClientRef;

pub mod id;
pub mod get;

/// Statistics endpoint handler builder.
#[derive(Clone, Debug)]
pub struct ChapterEndpoint {
    http_client: HttpClientRef,
}

impl ChapterEndpoint {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }

    
}