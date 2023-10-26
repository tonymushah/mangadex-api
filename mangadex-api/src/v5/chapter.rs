//! Chapter endpoint handler.
//!
//! <https://api.mangadex.org/swagger.html#/Chapter>

pub mod get;
pub mod id;

use uuid::Uuid;

use crate::v5::chapter::get::ListChapterBuilder;
use crate::v5::chapter::id::IdEndpoint;
use crate::HttpClientRef;

/// Chapter endpoint handler builder.
#[derive(Debug)]
pub struct ChapterBuilder {
    http_client: HttpClientRef,
}

impl ChapterBuilder {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }

    /// Search a list of chapters.
    ///
    /// <https://api.mangadex.org/swagger.html#/Chapter/get-chapter>
    pub fn get(&self) -> ListChapterBuilder {
        ListChapterBuilder::default().http_client(self.http_client.clone())
    }

    pub fn id(&self, id: Uuid) -> IdEndpoint {
        IdEndpoint::new(self.http_client.clone(), id)
    }
}
