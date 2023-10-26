//! MangaDex@Home endpoint handler.
//!
//! <https://api.mangadex.org/swagger.html#/AtHome>
pub mod server;

use crate::v5::at_home::server::ServerEndPoint;
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
    pub fn server(&self) -> ServerEndPoint {
        ServerEndPoint::new(self.http_client.clone())
    }
}
