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
            .http_client(self.http_client.clone())
            .manga_id(self.id)
    }
    pub fn commit(&self) -> CommitEndpoint {
        CommitEndpoint::new(self.http_client.clone(), self.id)
    }
}
