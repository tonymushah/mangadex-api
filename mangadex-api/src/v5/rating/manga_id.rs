pub mod delete;
pub mod post;

use crate::HttpClientRef;

use uuid::Uuid;

use delete::DeleteMangaRatingBuilder;
use post::CreateUpdateMangaRatingBuilder;

#[derive(Debug)]
pub struct MangaIdEndpoint {
    http_client: HttpClientRef,
    manga_id: Uuid,
}

impl MangaIdEndpoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef, manga_id: Uuid) -> Self {
        Self {
            http_client,
            manga_id,
        }
    }
    pub fn delete(&self) -> DeleteMangaRatingBuilder {
        DeleteMangaRatingBuilder::default()
            .http_client(self.http_client.clone())
            .manga_id(self.manga_id)
    }
    pub fn post(&self) -> CreateUpdateMangaRatingBuilder {
        CreateUpdateMangaRatingBuilder::default()
            .http_client(self.http_client.clone())
            .manga_id(self.manga_id)
    }
}
