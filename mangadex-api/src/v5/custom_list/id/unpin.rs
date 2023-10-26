use uuid::Uuid;

use crate::HttpClientRef;

pub mod post;
use post::UnPinCustomListBuilder;

#[derive(Debug, Clone)]
pub struct UnPinEndpoint {
    http_client: HttpClientRef,
    id: Uuid,
}

impl UnPinEndpoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef, id: Uuid) -> Self {
        Self { http_client, id }
    }
    pub fn post(&self) -> UnPinCustomListBuilder {
        UnPinCustomListBuilder::default()
            .http_client(self.http_client.clone())
            .list_id(self.id)
    }
}
