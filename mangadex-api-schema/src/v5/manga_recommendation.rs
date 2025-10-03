use mangadex_api_types::RelationshipType;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    TypedAttributes,
    v5::{MangaRecommendationCollection, MangaRecommendationObject},
};

/// Manga Recommendation attributes
///
/// Used only at `GET /manga/{id}/recommendation`
#[derive(Debug, Deserialize, Clone, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[non_exhaustive]
pub struct MangaRecommendationAttributes {
    /// The actual score
    ///
    /// Often a value between 0.0 to 1.0
    pub score: f32,
}

impl TypedAttributes for MangaRecommendationAttributes {
    const TYPE_: mangadex_api_types::RelationshipType = RelationshipType::MangaRecommendation;
}

impl MangaRecommendationObject {
    pub fn titles(&self) -> Vec<Uuid> {
        self.find_relationships(RelationshipType::Manga)
            .into_iter()
            .map(|r| r.id)
            .collect()
    }
}

impl MangaRecommendationCollection {
    // pub fn title_scored()
}
