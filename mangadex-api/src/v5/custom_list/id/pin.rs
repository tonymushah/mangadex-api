use uuid::Uuid;

use crate::HttpClientRef;

pub mod post;

use post::PinCustomListBuilder;

#[derive(Debug, Clone)]
pub struct PinEndpoint {
    http_client: HttpClientRef,
    id: Uuid,
}

impl PinEndpoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef, id: Uuid) -> Self {
        Self { http_client, id }
    }
    pub fn post(&self) -> PinCustomListBuilder {
        PinCustomListBuilder::default()
            .http_client(self.http_client.clone())
            .list_id(self.id)
    }
}
