use crate::HttpClientRef;

use uuid::Uuid;

use self::post::StartEditChapterSessionBuilder;

pub mod post;

/// Statistics endpoint handler builder.
#[derive(Clone, Debug)]
pub struct ChapterIdEndpoint {
    http_client: HttpClientRef,
    id: Uuid,
}

impl ChapterIdEndpoint {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef, id: Uuid) -> Self {
        Self { http_client, id }
    }
    pub fn post(&self) -> StartEditChapterSessionBuilder {
        StartEditChapterSessionBuilder::default()
            .chapter_id(self.id)
            .http_client(self.http_client.clone())
    }
}
