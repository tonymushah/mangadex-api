pub mod delete;
pub mod post;

use crate::HttpClientRef;

use uuid::Uuid;

use delete::RemoveMangaFromCustomListBuilder;
use post::AddMangaToCustomListBuilder;

#[derive(Debug)]
pub struct ListIdEndpoint {
    http_client: HttpClientRef,
    id: Uuid,
    list_id: Uuid,
}

impl ListIdEndpoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef, id: Uuid, list_id: Uuid) -> Self {
        Self {
            http_client,
            id,
            list_id,
        }
    }
    pub fn delete(&self) -> RemoveMangaFromCustomListBuilder {
        RemoveMangaFromCustomListBuilder::default()
            .http_client(self.http_client.clone())
            .manga_id(self.id)
            .list_id(self.list_id)
    }
    pub fn post(&self) -> AddMangaToCustomListBuilder {
        AddMangaToCustomListBuilder::default()
            .http_client(self.http_client.clone())
            .manga_id(self.id)
            .list_id(self.list_id)
    }
}
