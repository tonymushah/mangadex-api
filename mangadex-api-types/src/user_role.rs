use serde::{Deserialize, Serialize};

/// User roles that define what a user has permission to do.
/// More details at : https://api.mangadex.org/docs/static-data/#user-roles-enum
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum UserRole {
    /// MangaDex admins
    RoleAdmin,
    /// Banned
    RoleBanned,
    /// Helpers contributing by filling in missing information (Description, External Links) on Manga pages on MangaDex
    RoleContributor,
    /// Designer
    RoleDesigner,
    /// MangaDex site developers
    RoleDeveloper,
    /// Moderates the forum
    RoleForumModerator,

    RoleGlobalModerator,
    /// Leaders of active groups on MangaDex
    RoleGroupLeader,
    /// Member of a group
    RoleGroupMember,
    /// Users viewing the site without being logged in
    RoleGuest,
    /// Member of a group
    RoleMember,
    /// Involved with the [MangaDex@Home](mailto:MangaDex@Home) project
    RoleMdAtHome,
    /// Uploaded 500 or more chapters to MangaDex
    RolePowerUploader,
    ///Manages social media
    RolePublicRelations,
    /// Staff
    RoleStaff,
    /// Accounts that haven't had their email address verified yet
    RoleUnverified,
    /// A normal account
    RoleUser,
    /// Important people that in one way or another helped MangaDex
    RoleVip,
    #[serde(other)]
    Unknown
}
