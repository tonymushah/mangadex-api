//! Statistics endpoint handler.
//!
//! <https://api.mangadex.org/swagger.html#/Statistics>

pub mod chapter;
pub mod group;
pub mod manga;

use crate::HttpClientRef;

use self::{chapter::ChapterEndpoint, group::GroupEndpoint, manga::MangaEndpoint};

/// Statistics endpoint handler builder.
#[derive(Clone, Debug)]
pub struct StatisticsBuilder {
    http_client: HttpClientRef,
}

impl StatisticsBuilder {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }
    pub fn chapter(&self) -> ChapterEndpoint {
        ChapterEndpoint::new(self.http_client.clone())
    }
    pub fn group(&self) -> GroupEndpoint {
        GroupEndpoint::new(self.http_client.clone())
    }
    pub fn manga(&self) -> MangaEndpoint {
        MangaEndpoint::new(self.http_client.clone())
    }
}
