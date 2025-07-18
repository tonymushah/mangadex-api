use mangadex_api_types::{MangaRelation, RelationshipType};
use serde::Deserialize;

use crate::TypedAttributes;

/// Response struct for the manga relation list endpoint (GET `/manga/:id/relation`).
#[derive(Clone, Debug, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[non_exhaustive]
pub struct MangaRelationAttributes {
    pub relation: MangaRelation,
    pub version: u32,
}

impl TypedAttributes for MangaRelationAttributes {
    const TYPE_: mangadex_api_types::RelationshipType = RelationshipType::MangaRelation;
}
