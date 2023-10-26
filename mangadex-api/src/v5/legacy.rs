//! Legacy endpoint handler.
//!
//! <https://api.mangadex.org/swagger.html#/Legacy>

pub mod mapping;

use crate::HttpClientRef;

use self::mapping::MappingEndpoint;

/// Legacy endpoint handler builder.
#[derive(Clone, Debug)]
pub struct LegacyBuilder {
    http_client: HttpClientRef,
}

impl LegacyBuilder {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }

    pub fn mapping(self) -> MappingEndpoint {
        MappingEndpoint::new(self.http_client)
    }
}
