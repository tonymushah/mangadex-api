//! Cover art endpoint handler.
//!
//! <https://api.mangadex.org/swagger.html#/Cover>

pub mod manga_id;
pub mod cover_id;
pub mod get;

use uuid::Uuid;

use crate::v5::cover::get::ListCoverBuilder;
use crate::HttpClientRef;

use self::cover_id::CoverIdEndpoint;
use self::manga_id::MangaIdEndpoint;

/// Cover art endpoint handler builder.
#[derive(Debug)]
pub struct CoverBuilder {
    http_client: HttpClientRef,
}

impl CoverBuilder {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }

    /// Search a list of cover art.
    ///
    /// <https://api.mangadex.org/swagger.html#/Cover/get-cover>
    pub fn get(&self) -> ListCoverBuilder {
        ListCoverBuilder::default().http_client(self.http_client.clone())
    }

    pub fn cover_id(&self, cover_id : Uuid) -> CoverIdEndpoint{
        CoverIdEndpoint::new(self.http_client.clone(), cover_id)
    }
    pub fn manga_id(&self, manga_id : Uuid) -> MangaIdEndpoint{
        MangaIdEndpoint::new(self.http_client.clone(), manga_id)
    }
}
