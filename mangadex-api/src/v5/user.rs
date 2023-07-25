//! User endpoint handler.
//!
//! <https://api.mangadex.org/swagger.html#/User>

#[cfg(not(feature = "deserializable-endpoint"))]
#[deprecated = "Usage deprecated after the introduction of OAuth authentification from Mangadex API 5.9"]
#[cfg(feature = "legacy-account")]
mod approve_deletion;
#[cfg(not(feature = "deserializable-endpoint"))]
pub(crate) mod custom_lists;
#[cfg(not(feature = "deserializable-endpoint"))]
#[cfg(feature = "legacy-account")]
mod delete;
#[cfg(not(feature = "deserializable-endpoint"))]
mod followed_custom_lists;
#[cfg(not(feature = "deserializable-endpoint"))]
pub(crate) mod followed_groups;
#[cfg(not(feature = "deserializable-endpoint"))]
mod followed_manga;
#[cfg(not(feature = "deserializable-endpoint"))]
pub(crate) mod followed_manga_feed;
#[cfg(not(feature = "deserializable-endpoint"))]
mod followed_users;
#[cfg(not(feature = "deserializable-endpoint"))]
mod get;
#[cfg(not(feature = "deserializable-endpoint"))]
mod is_following_custom_list;
#[cfg(not(feature = "deserializable-endpoint"))]
mod is_following_group;
#[cfg(not(feature = "deserializable-endpoint"))]
mod is_following_manga;
#[cfg(not(feature = "deserializable-endpoint"))]
mod is_following_user;
#[cfg(not(feature = "deserializable-endpoint"))]
mod list;
#[cfg(not(feature = "deserializable-endpoint"))]
mod me;
#[cfg(not(feature = "deserializable-endpoint"))]
pub(crate) mod my_custom_lists;
#[cfg(not(feature = "deserializable-endpoint"))]
#[cfg(feature = "legacy-account")]
mod update_email;
#[cfg(not(feature = "deserializable-endpoint"))]
#[cfg(feature = "legacy-account")]
mod update_password;

#[cfg(feature = "deserializable-endpoint")]
#[deprecated = "Usage deprecated after the introduction of OAuth authentification from Mangadex API 5.9"]
#[cfg(feature = "legacy-account")]
pub mod approve_deletion;
#[cfg(feature = "deserializable-endpoint")]
pub mod custom_lists;
#[cfg(feature = "deserializable-endpoint")]
#[cfg(feature = "legacy-account")]
pub mod delete;
#[cfg(feature = "deserializable-endpoint")]
pub mod followed_custom_lists;
#[cfg(feature = "deserializable-endpoint")]
pub mod followed_groups;
#[cfg(feature = "deserializable-endpoint")]
pub mod followed_manga;
#[cfg(feature = "deserializable-endpoint")]
pub mod followed_manga_feed;
#[cfg(feature = "deserializable-endpoint")]
pub mod followed_users;
#[cfg(feature = "deserializable-endpoint")]
pub mod get;
#[cfg(feature = "deserializable-endpoint")]
pub mod is_following_custom_list;
#[cfg(feature = "deserializable-endpoint")]
pub mod is_following_group;
#[cfg(feature = "deserializable-endpoint")]
pub mod is_following_manga;
#[cfg(feature = "deserializable-endpoint")]
pub mod is_following_user;
#[cfg(feature = "deserializable-endpoint")]
pub mod list;
#[cfg(feature = "deserializable-endpoint")]
pub mod me;
#[cfg(feature = "deserializable-endpoint")]
pub mod my_custom_lists;
#[cfg(feature = "deserializable-endpoint")]
#[cfg(feature = "legacy-account")]
pub mod update_email;
#[cfg(feature = "deserializable-endpoint")]
#[cfg(feature = "legacy-account")]
pub mod update_password;

#[cfg(feature = "legacy-account")]
use crate::v5::user::approve_deletion::ApproveUserDeletionBuilder;
use crate::v5::user::custom_lists::UserCustomListsBuilder;
#[cfg(feature = "legacy-account")]
use crate::v5::user::delete::DeleteUserBuilder;
use crate::v5::user::followed_custom_lists::GetFollowedCustomListsBuilder;
use crate::v5::user::followed_groups::FollowedGroupsBuilder;
use crate::v5::user::followed_manga::FollowedMangaBuilder;
use crate::v5::user::followed_manga_feed::GetFollowedMangaFeedBuilder;
use crate::v5::user::followed_users::FollowedUsersBuilder;
use crate::v5::user::get::GetUserBuilder;
use crate::v5::user::is_following_custom_list::IsFollowingCustomListBuilder;
use crate::v5::user::is_following_group::IsFollowingGroupBuilder;
use crate::v5::user::is_following_manga::IsFollowingMangaBuilder;
use crate::v5::user::is_following_user::IsFollowingUserBuilder;
use crate::v5::user::list::ListUserBuilder;
use crate::v5::user::me::GetMyUserDetailsBuilder;
use crate::v5::user::my_custom_lists::MyCustomListsBuilder;
#[cfg(feature = "legacy-account")]
use crate::v5::user::update_email::UpdateUserEmailBuilder;
#[cfg(feature = "legacy-account")]
use crate::v5::user::update_password::UpdateUserPasswordBuilder;
use crate::HttpClientRef;

/// User endpoint handler builder.
#[derive(Debug)]
pub struct UserBuilder {
    http_client: HttpClientRef,
}

impl UserBuilder {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }

    /// Search for users.
    ///
    /// <https://api.mangadex.org/swagger.html#/User/get-user>
    pub fn list(&self) -> ListUserBuilder {
        ListUserBuilder::default().http_client(self.http_client.clone())
    }

    /// Search for users.
    ///
    /// <https://api.mangadex.org/swagger.html#/User/get-user>
    ///
    /// This is an alias for the [`Self::list()`] function.
    pub fn search(&self) -> ListUserBuilder {
        self.list()
    }

    /// Get a single user.
    ///
    /// <https://api.mangadex.org/swagger.html#/User/get-user-id>
    pub fn get(&self) -> GetUserBuilder {
        GetUserBuilder::default().http_client(self.http_client.clone())
    }

    /// Get a single user.
    ///
    /// <https://api.mangadex.org/swagger.html#/User/get-user-id>
    ///
    /// This is an alias for [`Self::get()`] to maintain backwards-compatibility.
    pub fn view(&self) -> GetUserBuilder {
        self.get()
    }

    /// Get the manga feed (chapter list) of manga the logged-in user follows.
    ///
    /// <https://api.mangadex.org/swagger.html#/Feed/get-user-follows-manga-feed>
    pub fn followed_manga_feed(&self) -> GetFollowedMangaFeedBuilder {
        GetFollowedMangaFeedBuilder::default().http_client(self.http_client.clone())
    }

    /// Get the followed scanlation groups for the logged-in user.
    ///
    /// <https://api.mangadex.org/swagger.html#/Follows/get-user-follows-group>
    pub fn followed_groups(&self) -> FollowedGroupsBuilder {
        FollowedGroupsBuilder::default().http_client(self.http_client.clone())
    }

    /// Delete a user.
    ///
    /// <https://api.mangadex.org/swagger.html#/User/delete-user-id>
    #[deprecated = "Usage deprecated after the introduction of OAuth authentification from Mangadex API 5.9"]
    #[cfg(feature = "legacy-account")]
    pub fn delete(&self) -> DeleteUserBuilder {
        DeleteUserBuilder::default().http_client(self.http_client.clone())
    }

    /// Approve the deletion of a user.
    ///
    /// <https://api.mangadex.org/swagger.html#/User/post-user-delete-code>
    #[deprecated = "Usage deprecated after the introduction of OAuth authentification from Mangadex API 5.9"]
    #[cfg(feature = "legacy-account")]
    pub fn approve_deletion(&self) -> ApproveUserDeletionBuilder {
        ApproveUserDeletionBuilder::default().http_client(self.http_client.clone())
    }

    /// Update the logged-in user's password.
    ///
    /// <https://api.mangadex.org/swagger.html#/User/post-user-password>
    #[deprecated = "Usage deprecated after the introduction of OAuth authentification from Mangadex API 5.9"]
    #[cfg(feature = "legacy-account")]
    pub fn update_password(&self) -> UpdateUserPasswordBuilder {
        UpdateUserPasswordBuilder::default().http_client(self.http_client.clone())
    }

    /// Update the logged-in user's email.
    ///
    /// <https://api.mangadex.org/swagger.html#/User/post-user-email>
    #[deprecated = "Usage deprecated after the introduction of OAuth authentification from Mangadex API 5.9"]
    #[cfg(feature = "legacy-account")]
    pub fn update_email(&self) -> UpdateUserEmailBuilder {
        UpdateUserEmailBuilder::default().http_client(self.http_client.clone())
    }

    /// Get the logged-in user's details.
    ///
    /// <https://api.mangadex.org/swagger.html#/User/get-user-me>
    pub fn me(&self) -> GetMyUserDetailsBuilder {
        GetMyUserDetailsBuilder::default().http_client(self.http_client.clone())
    }

    /// Check if the logged-in user follows a given group.
    ///
    /// <https://api.mangadex.org/swagger.html#/Follows/get-user-follows-group-id>
    pub fn is_following_group(&self) -> IsFollowingGroupBuilder {
        IsFollowingGroupBuilder::default().http_client(self.http_client.clone())
    }

    /// Get the followed users for the logged-in user.
    ///
    /// <https://api.mangadex.org/swagger.html#/Follows/get-user-follows-user>
    pub fn followed_users(&self) -> FollowedUsersBuilder {
        FollowedUsersBuilder::default().http_client(self.http_client.clone())
    }

    /// Check if the logged-in user follows a given user.
    ///
    /// <https://api.mangadex.org/swagger.html#/Follows/get-user-follows-user-id>
    pub fn is_following_user(&self) -> IsFollowingUserBuilder {
        IsFollowingUserBuilder::default().http_client(self.http_client.clone())
    }

    /// Get the followed manga for the logged-in user.
    ///
    /// <https://api.mangadex.org/swagger.html#/Follows/get-user-follows-manga>
    pub fn followed_manga(&self) -> FollowedMangaBuilder {
        FollowedMangaBuilder::default().http_client(self.http_client.clone())
    }

    /// Check if the logged-in user follows a given manga.
    ///
    /// <https://api.mangadex.org/swagger.html#/Follows/get-user-follows-manga-id>
    pub fn is_following_manga(&self) -> IsFollowingMangaBuilder {
        IsFollowingMangaBuilder::default().http_client(self.http_client.clone())
    }

    /// Get the custom lists for the logged-in user.
    ///
    /// This will fetch public and private lists.
    ///
    /// <https://api.mangadex.org/swagger.html#/CustomList/get-user-list>
    pub fn my_custom_lists(&self) -> MyCustomListsBuilder {
        MyCustomListsBuilder::default().http_client(self.http_client.clone())
    }

    /// Get a user's public custom lists.
    ///
    /// <https://api.mangadex.org/swagger.html#/User/get-user-id-list>
    ///
    /// Private lists are not included.
    pub fn custom_lists(&self) -> UserCustomListsBuilder {
        UserCustomListsBuilder::default().http_client(self.http_client.clone())
    }

    /// Get the logged-in user's followed custom lists.
    ///
    /// <https://api.mangadex.org/swagger.html#/Follows/get-user-follows-list>
    pub fn followed_custom_lists(&self) -> GetFollowedCustomListsBuilder {
        GetFollowedCustomListsBuilder::default().http_client(self.http_client.clone())
    }

    /// Check if the logged-in user follows a custom list.
    ///
    /// <https://api.mangadex.org/swagger.html#/Follows/get-user-follows-list-id>
    pub fn is_following_custom_list(&self) -> IsFollowingCustomListBuilder {
        IsFollowingCustomListBuilder::default().http_client(self.http_client.clone())
    }
}
