use std::fmt::{Debug, Display};

use serde::{Deserialize, Serialize};

// Relationship types for response objects.
///
/// <https://api.mangadex.org/docs/3-enumerations/#relationship-types>
///
/// This should only be used with the `type` response field.
/// For use with the `includes[]` query parameter, refer to the [`ReferenceExpansionResource` enum](crate::ReferenceExpansionResource).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::Enum))]
pub enum RelationshipType {
    /// Manga resource.
    Manga,
    /// Chapter resource.
    Chapter,
    /// A Cover Art for a manga.
    ///
    /// On manga resources, only one cover art resource relation is returned,
    /// marking the primary cover if there are more than one. By default, this will be the latest
    /// volume's cover art. To see all the covers for a given manga, use the cover search endpoint.
    CoverArt,
    /// Author resource.
    Author,
    /// Author resource (drawers only).
    Artist,
    /// ScanlationGroup resource.
    ScanlationGroup,
    /// Tag resource.
    Tag,
    /// User resource.
    User,
    /// CustomList resource.
    CustomList,

    /// Legacy, numerical, ID to UUID mapping.
    MappingId,
    /// Scanlation group owner.
    Leader,
    /// Scanlation group member.
    Member,
    ReportReason,
    Report,
    UploadSession,
    UploadSessionFile,
    Collection,
    MangaRelation,
    Creator,
    Thread,
    ApiClient,
    SettingsTemplate,
    MangaRecommendation,
    /// Unsupported resource.
    ///
    /// This is not used by MangaDex, but this library, in case new types appear before the library
    /// is updated.
    #[serde(other)]
    Unknown,
}

impl Default for RelationshipType {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Display for RelationshipType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}
