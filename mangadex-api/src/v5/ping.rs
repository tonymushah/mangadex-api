//! Infrastructure endpoint handler.
//!
//! <https://api.mangadex.org/swagger.html#/Infrastructure>

pub mod get;

use get::PingBuilder;

use crate::HttpClientRef;

/// Legacy endpoint handler builder.
#[derive(Clone, Debug)]
pub struct PingEndpointBuilder {
    http_client: HttpClientRef,
}

impl PingEndpointBuilder {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }

    /// Ping the server.
    ///
    /// <https://api.mangadex.org/swagger.html#/Infrastructure/get_ping>
    pub fn get(self) -> PingBuilder {
        PingBuilder::default().http_client(self.http_client)
    }
}
