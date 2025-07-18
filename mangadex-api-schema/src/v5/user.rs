use mangadex_api_types::{RelationshipType, UserRole};
use serde::Deserialize;

use crate::TypedAttributes;

/// General user information.
#[derive(Clone, Debug, Deserialize, Default)]
#[non_exhaustive]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct UserAttributes {
    pub username: String,
    pub roles: Vec<UserRole>,
    pub version: u32,
}

impl TypedAttributes for UserAttributes {
    const TYPE_: mangadex_api_types::RelationshipType = RelationshipType::User;
}
