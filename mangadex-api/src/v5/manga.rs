//! Manga endpoint handler.
//!
//! <https://api.mangadex.org/swagger.html#/Manga>
#[cfg(not(feature = "deserializable-endpoint"))]
pub(crate) mod add_to_custom_list;
#[cfg(not(feature = "deserializable-endpoint"))]
mod aggregate;
#[cfg(not(feature = "deserializable-endpoint"))]
mod create;
#[cfg(not(feature = "deserializable-endpoint"))]
mod create_relation;
#[cfg(not(feature = "deserializable-endpoint"))]
mod delete;
#[cfg(not(feature = "deserializable-endpoint"))]
mod delete_relation;
#[cfg(not(feature = "deserializable-endpoint"))]
mod feed;
#[cfg(not(feature = "deserializable-endpoint"))]
mod follow;
#[cfg(not(feature = "deserializable-endpoint"))]
mod get;
#[cfg(not(feature = "deserializable-endpoint"))]
mod get_draft;
#[cfg(not(feature = "deserializable-endpoint"))]
mod get_manga_read_chapters;
#[cfg(not(feature = "deserializable-endpoint"))]
mod get_read_chapters;
#[cfg(not(feature = "deserializable-endpoint"))]
pub(crate) mod list;
#[cfg(not(feature = "deserializable-endpoint"))]
mod list_drafts;
#[cfg(not(feature = "deserializable-endpoint"))]
mod list_relations;
#[cfg(not(feature = "deserializable-endpoint"))]
mod list_tags;
#[cfg(not(feature = "deserializable-endpoint"))]
mod random;
#[cfg(not(feature = "deserializable-endpoint"))]
mod reading_status;
#[cfg(not(feature = "deserializable-endpoint"))]
mod reading_statuses;
#[cfg(not(feature = "deserializable-endpoint"))]
pub(crate) mod remove_from_custom_list;
#[cfg(not(feature = "deserializable-endpoint"))]
mod submit_draft;
#[cfg(not(feature = "deserializable-endpoint"))]
mod unfollow;
#[cfg(not(feature = "deserializable-endpoint"))]
mod update;
#[cfg(not(feature = "deserializable-endpoint"))]
mod update_reading_status;

#[cfg(feature = "deserializable-endpoint")]
pub mod add_to_custom_list;
#[cfg(feature = "deserializable-endpoint")]
pub mod aggregate;
#[cfg(feature = "deserializable-endpoint")]
pub mod create;
#[cfg(feature = "deserializable-endpoint")]
pub mod create_relation;
#[cfg(feature = "deserializable-endpoint")]
pub mod delete;
#[cfg(feature = "deserializable-endpoint")]
pub mod delete_relation;
#[cfg(feature = "deserializable-endpoint")]
pub mod feed;
#[cfg(feature = "deserializable-endpoint")]
pub mod follow;
#[cfg(feature = "deserializable-endpoint")]
pub mod get;
#[cfg(feature = "deserializable-endpoint")]
pub mod get_draft;
#[cfg(feature = "deserializable-endpoint")]
pub mod get_manga_read_chapters;
#[cfg(feature = "deserializable-endpoint")]
pub mod get_read_chapters;
#[cfg(feature = "deserializable-endpoint")]
pub mod list;
#[cfg(feature = "deserializable-endpoint")]
pub mod list_drafts;
#[cfg(feature = "deserializable-endpoint")]
pub mod list_relations;
#[cfg(feature = "deserializable-endpoint")]
pub mod list_tags;
#[cfg(feature = "deserializable-endpoint")]
pub mod random;
#[cfg(feature = "deserializable-endpoint")]
pub mod reading_status;
#[cfg(feature = "deserializable-endpoint")]
pub mod reading_statuses;
#[cfg(feature = "deserializable-endpoint")]
pub mod remove_from_custom_list;
#[cfg(feature = "deserializable-endpoint")]
pub mod submit_draft;
#[cfg(feature = "deserializable-endpoint")]
pub mod unfollow;
#[cfg(feature = "deserializable-endpoint")]
pub mod update;
#[cfg(feature = "deserializable-endpoint")]
pub mod update_reading_status;

use crate::v5::manga::add_to_custom_list::AddMangaToCustomListBuilder;
use crate::v5::manga::aggregate::GetMangaAggregateBuilder;
use crate::v5::manga::create::CreateMangaBuilder;
use crate::v5::manga::create_relation::CreateMangaRelationBuilder;
use crate::v5::manga::delete::DeleteMangaBuilder;
use crate::v5::manga::delete_relation::DeleteMangaRelationBuilder;
use crate::v5::manga::feed::GetMangaFeedBuilder;
use crate::v5::manga::follow::FollowMangaBuilder;
use crate::v5::manga::get::GetMangaBuilder;
use crate::v5::manga::get_draft::GetMangaDraftBuilder;
use crate::v5::manga::get_manga_read_chapters::GetMangaReadChaptersBuilder;
use crate::v5::manga::get_read_chapters::GetReadChaptersBuilder;
use crate::v5::manga::list::ListMangaBuilder;
use crate::v5::manga::list_drafts::ListMangaDraftsBuilder;
use crate::v5::manga::list_relations::ListMangaRelationsBuilder;
use crate::v5::manga::list_tags::ListTagsBuilder;
use crate::v5::manga::random::GetRandomMangaBuilder;
use crate::v5::manga::reading_status::MangaReadingStatusBuilder;
use crate::v5::manga::reading_statuses::MangaReadingStatusesBuilder;
use crate::v5::manga::remove_from_custom_list::RemoveMangaFromCustomListBuilder;
use crate::v5::manga::submit_draft::SubmitMangaDraftBuilder;
use crate::v5::manga::unfollow::UnfollowMangaBuilder;
use crate::v5::manga::update::UpdateMangaBuilder;
use crate::v5::manga::update_reading_status::UpdateMangaReadingStatusBuilder;
use crate::v5::user::followed_manga_feed::GetFollowedMangaFeedBuilder;
use crate::HttpClientRef;

/// Manga endpoint handler.
#[derive(Debug)]
pub struct MangaBuilder {
    http_client: HttpClientRef,
}

impl MangaBuilder {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }

    /// Search a list of Manga.
    ///
    /// <https://api.mangadex.org/swagger.html#/Manga/get-search-manga>
    pub fn list(&self) -> ListMangaBuilder {
        ListMangaBuilder::default().http_client(self.http_client.clone())
    }

    /// Search a list of Manga.
    ///
    /// <https://api.mangadex.org/swagger.html#/Manga/get-search-manga>
    ///
    /// This is an alias for the [`Self::list()`] function.
    pub fn search(&self) -> ListMangaBuilder {
        self.list()
    }

    /// Get Manga volumes and chapters.
    ///
    /// <https://api.mangadex.org/swagger.html#/Manga/get_manga__id__aggregate>
    pub fn aggregate(&self) -> GetMangaAggregateBuilder {
        GetMangaAggregateBuilder::default().http_client(self.http_client.clone())
    }

    /// View a single Manga.
    ///
    /// <https://api.mangadex.org/swagger.html#/Manga/get-manga-id>
    pub fn get(&self) -> GetMangaBuilder {
        GetMangaBuilder::default().http_client(self.http_client.clone())
    }

    /// View a single Manga.
    ///
    /// <https://api.mangadex.org/swagger.html#/Manga/get-manga-id>
    ///
    /// This is an alias for [`Self::get()`] to maintain backwards-compatibility.
    pub fn view(&self) -> GetMangaBuilder {
        self.get()
    }

    /// Update a manga.
    ///
    /// <https://api.mangadex.org/swagger.html#/Manga/put-manga-id>
    pub fn update(&self) -> UpdateMangaBuilder {
        UpdateMangaBuilder::default().http_client(self.http_client.clone())
    }

    /// Delete a manga.
    ///
    /// <https://api.mangadex.org/swagger.html#/Manga/delete-manga-id>
    pub fn delete(&self) -> DeleteMangaBuilder {
        DeleteMangaBuilder::default().http_client(self.http_client.clone())
    }

    /// Add manga to a custom list.
    ///
    /// <https://api.mangadex.org/swagger.html#/Manga/post-manga-id-list-listId>
    pub fn add_to_custom_list(&self) -> AddMangaToCustomListBuilder {
        AddMangaToCustomListBuilder::default().http_client(self.http_client.clone())
    }

    /// Remove manga from a custom list.
    ///
    /// <https://api.mangadex.org/swagger.html#/Manga/delete-manga-id-list-listId>
    pub fn remove_from_custom_list(&self) -> RemoveMangaFromCustomListBuilder {
        RemoveMangaFromCustomListBuilder::default().http_client(self.http_client.clone())
    }

    /// Get the manga feed (chapter list) of manga the logged-in user follows.
    ///
    /// <https://api.mangadex.org/swagger.html#/Feed/get-user-follows-manga-feed>
    ///
    /// Alias to [`MangaDexClient::user().followed_manga_feed()`](crate::v5::user::followed_manga_feed).
    pub fn followed_manga_feed(&self) -> GetFollowedMangaFeedBuilder {
        GetFollowedMangaFeedBuilder::default().http_client(self.http_client.clone())
    }

    /// Follow a manga for the logged-in user.
    ///
    /// <https://api.mangadex.org/swagger.html#/Manga/post-manga-id-follow>
    pub fn follow(&self) -> FollowMangaBuilder {
        FollowMangaBuilder::default().http_client(self.http_client.clone())
    }

    /// Unfollow a manga for the logged-in user.
    ///
    /// <https://api.mangadex.org/swagger.html#/Manga/delete-manga-id-follow>
    pub fn unfollow(&self) -> UnfollowMangaBuilder {
        UnfollowMangaBuilder::default().http_client(self.http_client.clone())
    }

    /// Get recent chapters for a Manga.
    ///
    /// <https://api.mangadex.org/swagger.html#/Manga/get-manga-id-feed>
    pub fn feed(&self) -> GetMangaFeedBuilder {
        GetMangaFeedBuilder::default().http_client(self.http_client.clone())
    }

    /// Get a random manga, chosen by MangaDex.
    ///
    /// <https://api.mangadex.org/swagger.html#/Manga/get-manga-random>
    pub fn random(&self) -> GetRandomMangaBuilder {
        GetRandomMangaBuilder::default().http_client(self.http_client.clone())
    }

    /// Create a new manga.
    ///
    /// <https://api.mangadex.org/swagger.html#/Manga/post-manga>
    pub fn create(&self) -> CreateMangaBuilder {
        CreateMangaBuilder::default().http_client(self.http_client.clone())
    }

    /// Get a list of chapter IDs that are marked as read for the specified manga.
    ///
    /// <https://api.mangadex.org/swagger.html#/Manga/get-manga-chapter-readmarkers>
    pub fn get_manga_read_chapters(&self) -> GetMangaReadChaptersBuilder {
        GetMangaReadChaptersBuilder::default().http_client(self.http_client.clone())
    }

    /// Get a list of chapter IDs that are marked as read for the given manga IDs.
    ///
    /// <https://api.mangadex.org/swagger.html#/Manga/get-manga-chapter-readmarkers-2>
    pub fn get_read_chapters(&self) -> GetReadChaptersBuilder {
        GetReadChaptersBuilder::default().http_client(self.http_client.clone())
    }

    /// List all of the available tags.
    ///
    /// <https://api.mangadex.org/swagger.html#/Manga/get-manga-tag>
    pub fn list_tags(&self) -> ListTagsBuilder {
        ListTagsBuilder::default().http_client(self.http_client.clone())
    }

    /// Get the reading status for a given followed manga.
    ///
    /// <https://api.mangadex.org/swagger.html#/Manga/get-manga-id-status>
    pub fn reading_status(&self) -> MangaReadingStatusBuilder {
        MangaReadingStatusBuilder::default().http_client(self.http_client.clone())
    }

    /// Get the reading statuses for all followed manga for the logged-in user.
    ///
    /// <https://api.mangadex.org/swagger.html#/Manga/get-manga-status>
    pub fn reading_statuses(&self) -> MangaReadingStatusesBuilder {
        MangaReadingStatusesBuilder::default().http_client(self.http_client.clone())
    }

    /// Update the reading status for a manga.
    ///
    /// Using a `None` (`null`) value in the `status` field will remove the reading status.
    ///
    /// <https://api.mangadex.org/swagger.html#/Manga/post-manga-id-status>
    pub fn update_reading_status(&self) -> UpdateMangaReadingStatusBuilder {
        UpdateMangaReadingStatusBuilder::default().http_client(self.http_client.clone())
    }

    /// Get a specific Manga Draft.
    ///
    /// <https://api.mangadex.org/swagger.html#/Manga/get-manga-id-draft>
    pub fn get_draft(&self) -> GetMangaDraftBuilder {
        GetMangaDraftBuilder::default().http_client(self.http_client.clone())
    }

    /// Submit a Manga Draft.
    ///
    /// A Manga Draft that is to be submitted must have at least one cover, must be in the "draft" state and must be passed the correct version in the request body.
    ///
    /// <https://api.mangadex.org/swagger.html#/Manga/commit-manga-draft>
    pub fn submit_draft(&self) -> SubmitMangaDraftBuilder {
        SubmitMangaDraftBuilder::default().http_client(self.http_client.clone())
    }

    /// Get a list of Manga Drafts.
    ///
    /// <https://api.mangadex.org/swagger.html#/Manga/get-manga-drafts>
    pub fn list_drafts(&self) -> ListMangaDraftsBuilder {
        ListMangaDraftsBuilder::default().http_client(self.http_client.clone())
    }

    /// Get a list of Manga Drafts.
    ///
    /// This is an alias for the [`Self::list_drafts()`] function.
    ///
    /// <https://api.mangadex.org/swagger.html#/Manga/get-manga-drafts>
    pub fn search_drafts(&self) -> ListMangaDraftsBuilder {
        self.list_drafts()
    }

    /// Get a list of a Manga's relations.
    ///
    /// <https://api.mangadex.org/swagger.html#/Manga/get-manga-relation>
    pub fn list_relations(&self) -> ListMangaRelationsBuilder {
        ListMangaRelationsBuilder::default().http_client(self.http_client.clone())
    }

    /// Create a Manga relation.
    ///
    /// <https://api.mangadex.org/swagger.html#/Manga/post-manga-relation>
    pub fn create_relation(&self) -> CreateMangaRelationBuilder {
        CreateMangaRelationBuilder::default().http_client(self.http_client.clone())
    }

    /// Delete a Manga relation.
    ///
    /// <https://api.mangadex.org/swagger.html#/Manga/delete-manga-relation-id>
    pub fn delete_relation(&self) -> DeleteMangaRelationBuilder {
        DeleteMangaRelationBuilder::default().http_client(self.http_client.clone())
    }
}
