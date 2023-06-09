#[derive(Debug, Clone)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct IsFollowingResponse {
    pub is_following: bool,
}
