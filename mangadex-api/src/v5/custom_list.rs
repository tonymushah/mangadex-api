//! CustomList endpoint handler.
//!
//! <https://api.mangadex.org/swagger.html#/CustomList>

mod create;
mod delete;
mod follow;
mod get;
pub(crate) mod manga_feed;
mod unfollow;
mod update;

use crate::v5::custom_list::create::CreateCustomListBuilder;
use crate::v5::custom_list::delete::DeleteCustomListBuilder;
use crate::v5::custom_list::follow::FollowCustomListBuilder;
use crate::v5::custom_list::get::GetCustomListBuilder;
use crate::v5::custom_list::manga_feed::CustomListMangaFeedBuilder;
use crate::v5::custom_list::unfollow::UnfollowCustomListBuilder;
use crate::v5::custom_list::update::UpdateCustomListBuilder;
use crate::v5::manga::add_to_custom_list::AddMangaToCustomListBuilder;
use crate::v5::manga::remove_from_custom_list::RemoveMangaFromCustomListBuilder;
use crate::v5::user::custom_lists::UserCustomListsBuilder;
use crate::v5::user::my_custom_lists::MyCustomListsBuilder;
use crate::HttpClientRef;

/// CustomList endpoint handler builder.
#[derive(Debug)]
pub struct CustomListBuilder {
    http_client: HttpClientRef,
}

impl CustomListBuilder {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }

    /// Create a custom list.
    ///
    /// <https://api.mangadex.org/swagger.html#/CustomList/post-list>
    pub fn create(&self) -> CreateCustomListBuilder {
        CreateCustomListBuilder::default().http_client(self.http_client.clone())
    }

    /// Get a custom list.
    ///
    /// <https://api.mangadex.org/swagger.html#/CustomList/get-list-id>
    pub fn get(&self) -> GetCustomListBuilder {
        GetCustomListBuilder::default().http_client(self.http_client.clone())
    }

    /// Update a custom list.
    ///
    /// <https://api.mangadex.org/swagger.html#/CustomList/put-list-id>
    pub fn update(&self) -> UpdateCustomListBuilder {
        UpdateCustomListBuilder::default().http_client(self.http_client.clone())
    }

    /// Delete a custom list.
    ///
    /// <https://api.mangadex.org/swagger.html#/CustomList/delete-list-id>
    pub fn delete(&self) -> DeleteCustomListBuilder {
        DeleteCustomListBuilder::default().http_client(self.http_client.clone())
    }

    /// Get the custom lists for the logged-in user.
    ///
    /// This will fetch public and private lists.
    ///
    /// <https://api.mangadex.org/swagger.html#/CustomList/get-user-list>
    pub fn my_custom_lists(&self) -> MyCustomListsBuilder {
        MyCustomListsBuilder::default().http_client(self.http_client.clone())
    }

    /// Get the public custom lists for a given user.
    ///
    /// <https://api.mangadex.org/swagger.html#/CustomList/get-user-id-list>
    ///
    /// Private lists are not included.
    pub fn user_custom_lists(&self) -> UserCustomListsBuilder {
        UserCustomListsBuilder::default().http_client(self.http_client.clone())
    }

    /// Get the manga feed for a given custom list.
    ///
    /// <https://api.mangadex.org/swagger.html#/CustomList/get-list-id-feed>
    pub fn manga_feed(&self) -> CustomListMangaFeedBuilder {
        CustomListMangaFeedBuilder::default().http_client(self.http_client.clone())
    }

    /// Add manga to a custom list.
    ///
    /// <https://api.mangadex.org/swagger.html#/CustomList/post-manga-id-list-listId>
    pub fn add_manga(&self) -> AddMangaToCustomListBuilder {
        AddMangaToCustomListBuilder::default().http_client(self.http_client.clone())
    }

    /// Remove manga from a custom list.
    ///
    /// <https://api.mangadex.org/swagger.html#/CustomList/delete-manga-id-list-listId>
    pub fn remove_manga(&self) -> RemoveMangaFromCustomListBuilder {
        RemoveMangaFromCustomListBuilder::default().http_client(self.http_client.clone())
    }

    /// Follow a custom list for the current user.
    ///
    /// <https://api.mangadex.org/swagger.html#/CustomList/follow-list-id>
    pub fn follow(&self) -> FollowCustomListBuilder {
        FollowCustomListBuilder::default().http_client(self.http_client.clone())
    }

    /// Unfollow a custom list for the current user.
    ///
    /// <https://api.mangadex.org/swagger.html#/CustomList/unfollow-list-id>
    pub fn unfollow(&self) -> UnfollowCustomListBuilder {
        UnfollowCustomListBuilder::default().http_client(self.http_client.clone())
    }
}
