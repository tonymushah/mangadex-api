//! Rating endpoint handler.
//!
//! <https://api.mangadex.org/docs/swagger.html#/Rating/Rating>
pub mod get;
pub mod manga_id;

use crate::HttpClientRef;
use get::GetYourMangaRatingsBuilder;
use manga_id::MangaIdEndpoint;
use uuid::Uuid;

/// Rating endpoint handler builder.
#[derive(Clone, Debug)]
pub struct RatingBuilder {
    http_client: HttpClientRef,
}

impl RatingBuilder {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }
    pub fn get(&self) -> GetYourMangaRatingsBuilder {
        GetYourMangaRatingsBuilder::default().http_client(self.http_client.clone())
    }
    pub fn manga_id(&self, manga_id: Uuid) -> MangaIdEndpoint {
        MangaIdEndpoint::new(self.http_client.clone(), manga_id)
    }
}
