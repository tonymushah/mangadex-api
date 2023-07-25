//! Infrastructure endpoint handler.
//!
//! <https://api.mangadex.org/swagger.html#/Infrastructure>

#[cfg(not(feature = "deserializable-endpoint"))]
mod ping;

#[cfg(feature = "deserializable-endpoint")]
pub mod ping;

use crate::v5::infrastructure::ping::PingBuilder;
use crate::HttpClientRef;

/// Legacy endpoint handler builder.
#[derive(Clone, Debug)]
pub struct InfrastructureBuilder {
    http_client: HttpClientRef,
}

impl InfrastructureBuilder {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }

    /// Ping the server.
    ///
    /// <https://api.mangadex.org/swagger.html#/Infrastructure/get_ping>
    pub fn ping(self) -> PingBuilder {
        PingBuilder::default().http_client(self.http_client)
    }
}
