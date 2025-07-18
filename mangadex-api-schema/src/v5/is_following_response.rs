#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[non_exhaustive]
pub struct IsFollowingResponse {
    pub is_following: bool,
}
