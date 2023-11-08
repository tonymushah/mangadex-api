pub mod post;

use crate::HttpClientRef;
use post::UploadCoverBuilder;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct MangaIdEndpoint {
    http_client: HttpClientRef,
    manga_id: Uuid,
}

impl MangaIdEndpoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef, manga_id: Uuid) -> Self {
        Self {
            http_client,
            manga_id,
        }
    }
    pub fn post(&self) -> UploadCoverBuilder {
        UploadCoverBuilder::default()
            .manga_id(self.manga_id)
            .http_client(self.http_client.clone())
    }
}
