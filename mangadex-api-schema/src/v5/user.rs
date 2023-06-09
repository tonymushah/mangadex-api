use serde::{Deserialize};

/// General user information.
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct UserAttributes {
    pub username: String,
    // TODO: Map these roles to an enum.
    pub roles: Vec<String>,
    pub version: u32,
}
