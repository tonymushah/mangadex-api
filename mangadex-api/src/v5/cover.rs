//! Cover art endpoint handler.
//!
//! <https://api.mangadex.org/swagger.html#/Cover>

#[cfg(not(feature = "deserializable-endpoint"))]
mod delete;
#[cfg(not(feature = "deserializable-endpoint"))]
mod edit;
#[cfg(not(feature = "deserializable-endpoint"))]
mod get;
#[cfg(not(feature = "deserializable-endpoint"))]
pub(crate) mod list;
#[cfg(not(feature = "deserializable-endpoint"))]
pub(crate) mod upload;

#[cfg(feature = "deserializable-endpoint")]
pub mod delete;
#[cfg(feature = "deserializable-endpoint")]
pub mod edit;
#[cfg(feature = "deserializable-endpoint")]
pub mod get;
#[cfg(feature = "deserializable-endpoint")]
pub mod list;
#[cfg(feature = "deserializable-endpoint")]
pub mod upload;

use crate::v5::cover::delete::DeleteCoverBuilder;
use crate::v5::cover::edit::EditCoverBuilder;
use crate::v5::cover::get::GetCoverBuilder;
use crate::v5::cover::list::ListCoverBuilder;
use crate::v5::cover::upload::UploadCoverBuilder;
use crate::HttpClientRef;

/// Cover art endpoint handler builder.
#[derive(Debug)]
pub struct CoverBuilder {
    http_client: HttpClientRef,
}

impl CoverBuilder {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }

    /// Search a list of cover art.
    ///
    /// <https://api.mangadex.org/swagger.html#/Cover/get-cover>
    pub fn list(&self) -> ListCoverBuilder {
        ListCoverBuilder::default().http_client(self.http_client.clone())
    }

    /// Search a list of cover art.
    ///
    /// <https://api.mangadex.org/swagger.html#/Cover/get-cover>
    ///
    /// This is an alias for the [`Self::list()`] function.
    pub fn search(&self) -> ListCoverBuilder {
        self.list()
    }

    /// View a single cover.
    ///
    /// <https://api.mangadex.org/swagger.html#/Cover/get-cover-id>
    pub fn get(&self) -> GetCoverBuilder {
        GetCoverBuilder::default().http_client(self.http_client.clone())
    }

    /// View a single cover.
    ///
    /// <https://api.mangadex.org/swagger.html#/Cover/get-cover-id>
    ///
    /// This is an alias for [`Self::get()`] to maintain backwards-compatibility.
    pub fn view(&self) -> GetCoverBuilder {
        self.get()
    }

    /// Edit a cover.
    ///
    /// <https://api.mangadex.org/swagger.html#/Cover/edit-cover>
    pub fn edit(&self) -> EditCoverBuilder {
        EditCoverBuilder::default().http_client(self.http_client.clone())
    }

    /// Delete a cover.
    ///
    /// <https://api.mangadex.org/swagger.html#/Cover/delete-cover>
    pub fn delete(&self) -> DeleteCoverBuilder {
        DeleteCoverBuilder::default().http_client(self.http_client.clone())
    }

    /// Upload a cover.
    ///
    /// <https://api.mangadex.org/swagger.html#/Cover/upload-cover>
    pub fn upload(&self) -> UploadCoverBuilder {
        UploadCoverBuilder::default().http_client(self.http_client.clone())
    }
}
