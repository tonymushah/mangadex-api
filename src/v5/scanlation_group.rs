//! Scanlation group endpoint handler.
//!
//! <https://api.mangadex.org/swagger.html#/ScanlationGroup>

mod create;
mod delete;
mod follow;
mod get;
pub(crate) mod list;
mod unfollow;
mod update;

use crate::v5::scanlation_group::create::CreateGroupBuilder;
use crate::v5::scanlation_group::delete::DeleteGroupBuilder;
use crate::v5::scanlation_group::follow::FollowGroupBuilder;
use crate::v5::scanlation_group::get::GetGroupBuilder;
use crate::v5::scanlation_group::list::ListGroupBuilder;
use crate::v5::scanlation_group::unfollow::UnfollowGroupBuilder;
use crate::v5::scanlation_group::update::UpdateGroupBuilder;
use crate::v5::user::followed_groups::FollowedGroupsBuilder;
use crate::HttpClientRef;

/// Scanlation group endpoint handler builder.
#[derive(Debug)]
pub struct ScanlationGroupBuilder {
    http_client: HttpClientRef,
}

impl ScanlationGroupBuilder {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }

    /// Search a list of scanlation groups.
    ///
    /// <https://api.mangadex.org/swagger.html#/ScanlationGroup/get-search-group>
    pub fn list(&self) -> ListGroupBuilder {
        ListGroupBuilder::default().http_client(self.http_client.clone())
    }

    /// Search a list of scanlation groups.
    ///
    /// <https://api.mangadex.org/swagger.html#/ScanlationGroup/get-search-group>
    ///
    /// This is an alias for the [`Self::list()`] function.
    pub fn search(&self) -> ListGroupBuilder {
        self.list()
    }

    /// View a single scanlation group.
    ///
    /// <https://api.mangadex.org/swagger.html#/ScanlationGroup/get-group-id>
    pub fn get(&self) -> GetGroupBuilder {
        GetGroupBuilder::default().http_client(self.http_client.clone())
    }

    /// View a single scanlation group.
    ///
    /// <https://api.mangadex.org/swagger.html#/ScanlationGroup/get-group-id>
    ///
    /// This is an alias for [`Self::get()`] to maintain backwards-compatibility.
    pub fn view(&self) -> GetGroupBuilder {
        self.get()
    }

    /// Create a new scanlation group.
    ///
    /// <https://api.mangadex.org/swagger.html#/ScanlationGroup/post-group>
    pub fn create(&self) -> CreateGroupBuilder {
        CreateGroupBuilder::default().http_client(self.http_client.clone())
    }

    /// Update a scanlation group.
    ///
    /// <https://api.mangadex.org/swagger.html#/ScanlationGroup/put-group-id>
    pub fn update(&self) -> UpdateGroupBuilder {
        UpdateGroupBuilder::default().http_client(self.http_client.clone())
    }

    /// Delete a scanlation group.
    ///
    /// <https://api.mangadex.org/swagger.html#/ScanlationGroup/delete-group-id>
    pub fn delete(&self) -> DeleteGroupBuilder {
        DeleteGroupBuilder::default().http_client(self.http_client.clone())
    }

    /// Follow a scanlation group.
    ///
    /// <https://api.mangadex.org/swagger.html#/ScanlationGroup/post-group-id-follow>
    pub fn follow(&self) -> FollowGroupBuilder {
        FollowGroupBuilder::default().http_client(self.http_client.clone())
    }

    /// Unfollow a scanlation group.
    ///
    /// <https://api.mangadex.org/swagger.html#/ScanlationGroup/delete-group-id-follow>
    pub fn unfollow(&self) -> UnfollowGroupBuilder {
        UnfollowGroupBuilder::default().http_client(self.http_client.clone())
    }

    /// Get the followed scanlation groups for the logged-in user.
    ///
    /// <https://api.mangadex.org/swagger.html#/ScanlationGroup/get-user-follows-group>
    pub fn get_followed(&self) -> FollowedGroupsBuilder {
        FollowedGroupsBuilder::default().http_client(self.http_client.clone())
    }
}
