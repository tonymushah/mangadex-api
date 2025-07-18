use serde::Deserialize;

/// JWT that must be included with requests that require Bearer authentication.
#[derive(Debug, Deserialize, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[non_exhaustive]
pub struct AuthTokens {
    /// Token that lives for 15 minutes.
    pub session: String,
    /// Token that lives for 1 month; allows for refreshing without re-authenticating.
    pub refresh: String,
}
