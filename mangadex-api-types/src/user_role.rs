use serde::{Deserialize, Serialize};

/// User roles that define what a user has permission to do.
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum UserRole {
    RoleGroupLeader,
    RoleGroupMember,
    RoleMember,
}
