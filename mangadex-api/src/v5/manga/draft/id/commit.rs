pub mod post;

use crate::HttpClientRef;

use post::SubmitMangaDraftBuilder;
use uuid::Uuid;

#[derive(Debug)]
pub struct CommitEndpoint {
    http_client: HttpClientRef,
    id: Uuid,
}

impl CommitEndpoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef, id: Uuid) -> Self {
        Self { http_client, id }
    }
    pub fn post(&self) -> SubmitMangaDraftBuilder {
        SubmitMangaDraftBuilder::default()
            .http_client(self.http_client.clone())
            .manga_id(self.id)
    }
}
