pub mod relation;

use crate::HttpClientRef;

use uuid::Uuid;

use relation::RelationEndpoint;

#[derive(Debug)]
pub struct MangaIdEndpoint {
    http_client: HttpClientRef,
    manga_id: Uuid,
}

impl MangaIdEndpoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef, manga_id: Uuid) -> Self {
        Self { http_client, manga_id }
    }
    pub fn relation(&self) -> RelationEndpoint{
        RelationEndpoint::new(self.http_client.clone(), self.manga_id)
    }
}