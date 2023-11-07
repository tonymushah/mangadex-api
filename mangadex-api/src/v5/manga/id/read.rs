pub mod get;
pub mod post;

use crate::HttpClientRef;

use uuid::Uuid;

use get::GetMangaReadChaptersBuilder;
use post::MarkChapterBatchBuilder;

#[derive(Debug)]
pub struct ReadEndpoint {
    http_client: HttpClientRef,
    id: Uuid,
}

impl ReadEndpoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef, id: Uuid) -> Self {
        Self { http_client, id }
    }
    pub fn get(&self) -> GetMangaReadChaptersBuilder {
        GetMangaReadChaptersBuilder::default()
            .manga_id(self.id)
            .http_client(self.http_client.clone())
    }
    pub fn post(&self) -> MarkChapterBatchBuilder {
        MarkChapterBatchBuilder::default()
            .manga_id(self.id)
            .http_client(self.http_client.clone())
    }
}
