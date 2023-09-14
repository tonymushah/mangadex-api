pub mod get;
pub mod listId;

use crate::HttpClientRef;

use get::GetMangaCustomListsBuilder;
use listId::ListIdEndpoint;
use uuid::Uuid;

#[derive(Debug)]
pub struct ListEndpoint {
    http_client: HttpClientRef,
    id: Uuid,
}

impl ListEndpoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef, id: Uuid) -> Self {
        Self { http_client, id }
    }
    pub fn list_id(&self, list_id: Uuid) -> ListIdEndpoint {
        ListIdEndpoint::new(self.http_client.clone(), self.id, list_id)
    }
    pub fn get(&self) -> GetMangaCustomListsBuilder {
        GetMangaCustomListsBuilder::default()
            .http_client(self.http_client.clone())
            .manga_id(self.id)
    }
}
