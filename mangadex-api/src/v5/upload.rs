//! Upload endpoint handler.
//!
//! <https://api.mangadex.org/swagger.html#/Upload>

#[cfg(not(feature = "deserializable-endpoint"))]
mod abandon_session;
#[cfg(not(feature = "deserializable-endpoint"))]
mod commit_session;
#[cfg(not(feature = "deserializable-endpoint"))]
mod delete_image;
#[cfg(not(feature = "deserializable-endpoint"))]
mod delete_images;
#[cfg(not(feature = "deserializable-endpoint"))]
mod get_session;
#[cfg(not(feature = "deserializable-endpoint"))]
mod start_edit_chapter_session;
#[cfg(not(feature = "deserializable-endpoint"))]
mod start_session;
#[cfg(not(feature = "deserializable-endpoint"))]
mod upload_images;

#[cfg(feature = "deserializable-endpoint")]
pub mod abandon_session;
#[cfg(feature = "deserializable-endpoint")]
pub mod commit_session;
#[cfg(feature = "deserializable-endpoint")]
pub mod delete_image;
#[cfg(feature = "deserializable-endpoint")]
pub mod delete_images;
#[cfg(feature = "deserializable-endpoint")]
pub mod get_session;
#[cfg(feature = "deserializable-endpoint")]
pub mod start_edit_chapter_session;
#[cfg(feature = "deserializable-endpoint")]
pub mod start_session;
#[cfg(feature = "deserializable-endpoint")]
pub mod upload_images;

use uuid::Uuid;

use crate::v5::cover::manga_id::post::UploadCoverBuilder;
use crate::v5::upload::abandon_session::AbandonUploadSessionBuilder;
use crate::v5::upload::commit_session::CommitUploadSessionBuilder;
use crate::v5::upload::delete_image::DeleteImageBuilder;
use crate::v5::upload::delete_images::DeleteImagesBuilder;
use crate::v5::upload::get_session::GetUploadSessionBuilder;
use crate::v5::upload::start_edit_chapter_session::StartEditChapterSessionBuilder;
use crate::v5::upload::start_session::StartUploadSessionBuilder;
use crate::v5::upload::upload_images::UploadImagesBuilder;
use crate::HttpClientRef;

/// Upload endpoint handler builder.
#[derive(Debug)]
pub struct UploadBuilder {
    http_client: HttpClientRef,
}

impl UploadBuilder {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }

    /// Upload a manga cover image.
    ///
    /// <https://api.mangadex.org/swagger.html#/Upload/upload-cover>
    pub fn cover(&self, manga_id: Uuid) -> UploadCoverBuilder {
        UploadCoverBuilder::default().http_client(self.http_client.clone()).manga_id(manga_id)
    }

    /// Get the logged-in user's current upload session.
    ///
    /// <https://api.mangadex.org/swagger.html#/Upload/get-upload-session>
    pub fn get_session(&self) -> GetUploadSessionBuilder {
        GetUploadSessionBuilder::default().http_client(self.http_client.clone())
    }

    /// Start an upload session.
    ///
    /// <https://api.mangadex.org/swagger.html#/Upload/begin-upload-session>
    pub fn start_session(&self) -> StartUploadSessionBuilder {
        StartUploadSessionBuilder::default().http_client(self.http_client.clone())
    }

    /// Start an edit chapter session.
    ///
    /// <https://api.mangadex.org/swagger.html#/Upload/begin-edit-session>
    pub fn start_edit_chapter_session(&self) -> StartEditChapterSessionBuilder {
        StartEditChapterSessionBuilder::default().http_client(self.http_client.clone())
    }

    /// Upload images to the upload session.
    ///
    /// <https://api.mangadex.org/swagger.html#/Upload/put-upload-session-file>
    ///
    /// Currently, there is a maximum of 10 images per request.
    pub fn upload_images(&self) -> UploadImagesBuilder {
        UploadImagesBuilder::default().http_client(self.http_client.clone())
    }

    /// Abandon an ongoing upload session.
    ///
    /// <https://api.mangadex.org/swagger.html#/Upload/abandon-upload-session>
    pub fn abandon_session(&self) -> AbandonUploadSessionBuilder {
        AbandonUploadSessionBuilder::default().http_client(self.http_client.clone())
    }

    /// Commit the upload session and specify chapter data.
    ///
    /// <https://api.mangadex.org/swagger.html#/Upload/commit-upload-session>
    pub fn commit_session(&self) -> CommitUploadSessionBuilder {
        CommitUploadSessionBuilder::new(self.http_client.clone())
    }

    /// Delete an uploaded image from the upload session.
    ///
    /// <https://api.mangadex.org/swagger.html#/Upload/delete-uploaded-session-file>
    pub fn delete_image(&self) -> DeleteImageBuilder {
        DeleteImageBuilder::default().http_client(self.http_client.clone())
    }

    /// Delete a set of uploaded images from the upload session.
    ///
    /// <https://api.mangadex.org/swagger.html#/Upload/delete-uploaded-session-files>
    pub fn delete_images(&self) -> DeleteImagesBuilder {
        DeleteImagesBuilder::default().http_client(self.http_client.clone())
    }
}
