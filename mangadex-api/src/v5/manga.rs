//! Manga endpoint handler.
//!
//! <https://api.mangadex.org/swagger.html#/Manga>

use crate::HttpClientRef;

pub mod draft;
pub mod get;
pub mod id;
pub mod mangaId;
pub mod post;
pub mod random;
pub mod read;
pub mod status;
pub mod tag;

use draft::DraftEndpoint;
use get::ListMangaBuilder;
use uuid::Uuid;
use id::IdEndpoint;

/// Manga endpoint handler.
#[derive(Debug)]
pub struct MangaBuilder {
    http_client: HttpClientRef,
}

impl MangaBuilder {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }

    pub fn draft(&self) -> DraftEndpoint{
        DraftEndpoint::new(self.http_client.clone())
    }

    pub fn get(&self) -> ListMangaBuilder{
        ListMangaBuilder::default().http_client(self.http_client.clone())
    }
    
    pub fn id(&self, id: Uuid) -> IdEndpoint{
        IdEndpoint::new(self.http_client.clone(), id)
    }
}
