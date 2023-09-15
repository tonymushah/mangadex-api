pub mod delete;

use crate::HttpClientRef;

use uuid::Uuid;

use delete::DeleteMangaRelationBuilder;

#[derive(Debug)]
pub struct RelationIdEndpoint {
    http_client: HttpClientRef,
    manga_id: Uuid,
    id : Uuid
}

impl RelationIdEndpoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef, manga_id: Uuid, id: Uuid) -> Self {
        Self { http_client, manga_id, id }
    }
    pub fn delete(&self) -> DeleteMangaRelationBuilder{
        DeleteMangaRelationBuilder::default()
            .manga_id(self.manga_id)
            .relation_id(self.id)
            .http_client(self.http_client.clone())
    }
}