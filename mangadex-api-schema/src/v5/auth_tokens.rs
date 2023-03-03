use serde::Deserialize;

/// JWT that must be included with requests that require Bearer authentication.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct AuthTokens {
    /// Token that lives for 15 minutes.
    pub session: String,
    /// Token that lives for 1 month; allows for refreshing without re-authenticating.
    pub refresh: String,
}
