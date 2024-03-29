pub mod delete;
pub mod post;

use crate::HttpClientRef;

use delete::UnfollowMangaBuilder;
use post::FollowMangaBuilder;
use uuid::Uuid;

#[derive(Debug)]
pub struct FollowEndpoint {
    http_client: HttpClientRef,
    id: Uuid,
}

impl FollowEndpoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef, id: Uuid) -> Self {
        Self { http_client, id }
    }
    pub fn post(&self) -> FollowMangaBuilder {
        FollowMangaBuilder::default()
            .manga_id(self.id)
            .http_client(self.http_client.clone())
    }
    pub fn delete(&self) -> UnfollowMangaBuilder {
        UnfollowMangaBuilder::default()
            .manga_id(self.id)
            .http_client(self.http_client.clone())
    }
}
