//! Legacy endpoint handler.
//!
//! <https://api.mangadex.org/swagger.html#/Legacy>

#[cfg(not(feature = "deserializable-endpoint"))]
mod id_mapping;

#[cfg(feature = "deserializable-endpoint")]
pub mod id_mapping;

use crate::v5::legacy::id_mapping::LegacyIdMappingBuilder;
use crate::HttpClientRef;

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

    /// Get the UUID of a resource from legacy numerical IDs.
    ///
    /// <https://api.mangadex.org/swagger.html#/Legacy/post-legacy-mapping>
    pub fn id_mapping(self) -> LegacyIdMappingBuilder {
        LegacyIdMappingBuilder::default().http_client(self.http_client)
    }
}
