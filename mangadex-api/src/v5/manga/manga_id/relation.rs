pub mod get;
pub mod id;
pub mod post;

use crate::HttpClientRef;

use uuid::Uuid;

use get::ListMangaRelationsBuilder;
use id::RelationIdEndpoint;
use post::CreateMangaRelationBuilder;

#[derive(Debug)]
pub struct RelationEndpoint {
    http_client: HttpClientRef,
    manga_id: Uuid,
}

impl RelationEndpoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef, manga_id: Uuid) -> Self {
        Self {
            http_client,
            manga_id,
        }
    }
    pub fn get(&self) -> ListMangaRelationsBuilder {
        ListMangaRelationsBuilder::default()
            .manga_id(self.manga_id)
            .http_client(self.http_client.clone())
    }
    pub fn post(&self) -> CreateMangaRelationBuilder {
        CreateMangaRelationBuilder::default()
            .manga_id(self.manga_id)
            .http_client(self.http_client.clone())
    }
    pub fn id(&self, id: Uuid) -> RelationIdEndpoint {
        RelationIdEndpoint::new(self.http_client.clone(), self.manga_id, id)
    }
}
