//! MangaDex@Home endpoint handler.
//!
//! <https://api.mangadex.org/swagger.html#/AtHome>
#[cfg(not(feature = "deserializable-endpoint"))]
mod server;
#[cfg(feature = "deserializable-endpoint")]
pub mod server;

use crate::v5::at_home::server::GetAtHomeServerBuilder;
use crate::HttpClientRef;

/// MangaDex@Home endpoint handler builder.
#[derive(Debug)]
pub struct AtHomeBuilder {
    http_client: HttpClientRef,
}

impl AtHomeBuilder {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }

    /// Get a MangaDex@Home server URL.
    ///
    /// <https://api.mangadex.org/swagger.html#/AtHome/get-at-home-server-chapterId>
    ///
    /// This can be used to [fetch chapter pages](https://api.mangadex.org/swagger.html#/AtHome/Reading-a-chapter-using-the-API/Retrieving-pages-from-the-MangaDex@Home-network).
    pub fn server(&self) -> GetAtHomeServerBuilder {
        GetAtHomeServerBuilder::default().http_client(self.http_client.clone())
    }
}
