pub mod post;
pub mod chapter_id;

use crate::HttpClientRef;

use self::{post::StartUploadSessionBuilder, chapter_id::ChapterIdEndpoint};

/// Statistics endpoint handler builder.
#[derive(Clone, Debug)]
pub struct BeginEndpoint {
    http_client: HttpClientRef,
}

impl BeginEndpoint {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }

    pub fn post(&self) -> StartUploadSessionBuilder {
        StartUploadSessionBuilder::default().http_client(self.http_client.clone())
    }
    pub fn chapter_id(&self, id: uuid::Uuid) -> ChapterIdEndpoint {
        ChapterIdEndpoint::new(self.http_client.clone(), id)
    }
}