use mangadex_api_types::MangaRelation;
use serde::Deserialize;

/// Response struct for the manga relation list endpoint (GET `/manga/:id/aggregate`).
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct MangaRelationAttributes {
    pub relation: MangaRelation,
    pub version: u32,
}
