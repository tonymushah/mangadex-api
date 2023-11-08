pub mod get;
pub mod list_id;

use crate::HttpClientRef;

use get::GetMangaCustomListsBuilder;
use list_id::ListIdEndpoint;
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
            .manga_id(self.id)
            .http_client(self.http_client.clone())
    }
}
