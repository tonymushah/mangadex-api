//! Chapter endpoint handler.
//!
//! <https://api.mangadex.org/swagger.html#/Chapter>

mod delete;
mod get;
pub(crate) mod list;
pub(crate) mod mark_batch;
mod mark_read;
mod mark_unread;
mod update;

use crate::v5::chapter::delete::DeleteChapterBuilder;
use crate::v5::chapter::get::GetChapterBuilder;
use crate::v5::chapter::list::ListChapterBuilder;
use crate::v5::chapter::mark_batch::MarkChapterBatchBuilder;
use crate::v5::chapter::mark_read::MarkChapterReadBuilder;
use crate::v5::chapter::mark_unread::MarkChapterUnreadBuilder;
use crate::v5::chapter::update::UpdateChapterBuilder;
use crate::v5::user::followed_manga_feed::GetFollowedMangaFeedBuilder;
use crate::HttpClientRef;

/// Chapter endpoint handler builder.
#[derive(Debug)]
pub struct ChapterBuilder {
    http_client: HttpClientRef,
}

impl ChapterBuilder {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }

    /// Search a list of chapters.
    ///
    /// <https://api.mangadex.org/swagger.html#/Chapter/get-chapter>
    pub fn list(&self) -> ListChapterBuilder {
        ListChapterBuilder::default().http_client(self.http_client.clone())
    }

    /// Search a list of chapters.
    ///
    /// <https://api.mangadex.org/swagger.html#/Chapter/get-chapter>
    ///
    /// This is an alias for the [`Self::list()`] function.
    pub fn search(&self) -> ListChapterBuilder {
        self.list()
    }

    /// View a single chapter.
    ///
    /// <https://api.mangadex.org/swagger.html#/Chapter/get-chapter-id>
    pub fn get(&self) -> GetChapterBuilder {
        GetChapterBuilder::default().http_client(self.http_client.clone())
    }

    /// View a single chapter.
    ///
    /// <https://api.mangadex.org/swagger.html#/Chapter/get-chapter-id>
    ///
    /// This is an alias for [`Self::get()`] to maintain backwards-compatibility.
    pub fn view(&self) -> GetChapterBuilder {
        self.get()
    }

    /// Update a chapter.
    ///
    /// <https://api.mangadex.org/swagger.html#/Chapter/put-chapter-id>
    pub fn update(&self) -> UpdateChapterBuilder {
        UpdateChapterBuilder::default().http_client(self.http_client.clone())
    }

    /// Delete a chapter.
    ///
    /// <https://api.mangadex.org/swagger.html#/Chapter/delete-chapter-id>
    pub fn delete(&self) -> DeleteChapterBuilder {
        DeleteChapterBuilder::default().http_client(self.http_client.clone())
    }

    /// Mark multiple chapters for one manga as read and/or unread.
    ///
    /// <https://api.mangadex.org/swagger.html#/Chapter/post-manga-chapter-readmarkers>
    pub fn mark_batch(&self) -> MarkChapterBatchBuilder {
        MarkChapterBatchBuilder::default().http_client(self.http_client.clone())
    }

    /// Mark a chapter as read for the current user.
    ///
    /// <https://api.mangadex.org/swagger.html#/Chapter/chapter-id-read>
    pub fn mark_read(&self) -> MarkChapterReadBuilder {
        MarkChapterReadBuilder::default().http_client(self.http_client.clone())
    }

    /// Mark a chapter as unread for the current user.
    ///
    /// <https://api.mangadex.org/swagger.html#/Chapter/chapter-id-unread>
    pub fn mark_unread(&self) -> MarkChapterUnreadBuilder {
        MarkChapterUnreadBuilder::default().http_client(self.http_client.clone())
    }

    /// Get the manga feed (chapter list) of manga the logged-in user follows.
    ///
    /// <https://api.mangadex.org/swagger.html#/Feed/get-user-follows-manga-feed>
    ///
    /// Alias to [`MangaDexClient::user().followed_manga_feed()`](crate::v5::user::followed_manga_feed).
    pub fn followed_manga_feed(&self) -> GetFollowedMangaFeedBuilder {
        GetFollowedMangaFeedBuilder::default().http_client(self.http_client.clone())
    }
}
