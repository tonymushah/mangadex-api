pub mod chapter_id;
pub mod post;

use crate::HttpClientRef;

use self::{chapter_id::ChapterIdEndpoint, post::StartUploadSessionBuilder};

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
