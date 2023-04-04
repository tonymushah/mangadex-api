use serde::{Deserialize, Serialize};

/// General user information.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct UserAttributes {
    pub username: String,
    // TODO: Map these roles to an enum.
    pub roles: Vec<String>,
    pub version: u32,
}
