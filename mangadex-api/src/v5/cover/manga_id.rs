pub mod post;

use post::UploadCoverBuilder;
use uuid::Uuid;
use crate::HttpClientRef;

#[derive(Debug, Clone)]
pub struct MangaIdEndpoint{
    http_client: HttpClientRef,
    manga_id : Uuid
}


impl MangaIdEndpoint{
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef, manga_id: Uuid) -> Self{
        Self { http_client, manga_id }
    }
    pub fn post(&self) -> UploadCoverBuilder{
        UploadCoverBuilder::default().http_client(self.http_client.clone()).manga_id(self.manga_id)
    }
}
