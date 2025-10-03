use crate::v5::MangaRecommendationAttributes;

use super::{ApiData, ApiObjectStringId, Results};

pub type MangaRecommendationObject = ApiObjectStringId<MangaRecommendationAttributes>;
pub type MangaRecommendationData = ApiData<MangaRecommendationObject>;
pub type MangaRecommendationCollection = Results<MangaRecommendationObject>;
