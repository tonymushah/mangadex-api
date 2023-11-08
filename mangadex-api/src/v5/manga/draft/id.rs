pub mod commit;
pub mod get;

use crate::HttpClientRef;

use commit::CommitEndpoint;
use get::GetMangaDraftBuilder;
use uuid::Uuid;

#[derive(Debug)]
pub struct IdEndpoint {
    http_client: HttpClientRef,
    id: Uuid,
}

impl IdEndpoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef, id: Uuid) -> Self {
        Self { http_client, id }
    }
    pub fn get(&self) -> GetMangaDraftBuilder {
        GetMangaDraftBuilder::default()
            .manga_id(self.id)
            .http_client(self.http_client.clone())
    }
    pub fn commit(&self) -> CommitEndpoint {
        CommitEndpoint::new(self.http_client.clone(), self.id)
    }
}
