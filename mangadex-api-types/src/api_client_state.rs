use serde::{Deserialize, Serialize};

/// API Client state for approval.
///
/// The purpose of these are to discourage troll entries by requiring staff approval.
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum ApiClientState {
    Requested,
    Approved,
    Rejected,
    Autoapproved,
}
