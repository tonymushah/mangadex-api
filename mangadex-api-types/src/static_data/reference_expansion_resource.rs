use serde::{Deserialize, Serialize};

/// Relationship types for reference expansion.
///
/// <https://api.mangadex.org/docs/static-data/#relationship-types>
///
/// This should only be used with the `includes[]` query parameter.
/// For response types, refer to the [`RelationshipType` enum](crate::RelationshipType).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ReferenceExpansionResource {
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
}
