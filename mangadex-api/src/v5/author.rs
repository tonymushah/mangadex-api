//! Author endpoint handler.
//!
//! <https://api.mangadex.org/swagger.html#/Author>

#[cfg(not(feature = "deserializable-endpoint"))]
mod create;
#[cfg(not(feature = "deserializable-endpoint"))]
mod delete;
#[cfg(not(feature = "deserializable-endpoint"))]
mod get;
#[cfg(not(feature = "deserializable-endpoint"))]
pub(crate) mod list;
#[cfg(not(feature = "deserializable-endpoint"))]
mod update;

#[cfg(feature = "deserializable-endpoint")]
pub mod create;
#[cfg(feature = "deserializable-endpoint")]
pub mod delete;
#[cfg(feature = "deserializable-endpoint")]
pub mod get;
#[cfg(feature = "deserializable-endpoint")]
pub mod list;
#[cfg(feature = "deserializable-endpoint")]
pub mod update;

use crate::v5::author::create::CreateAuthorBuilder;
use crate::v5::author::delete::DeleteAuthorBuilder;
use crate::v5::author::get::GetAuthorBuilder;
use crate::v5::author::list::ListAuthorBuilder;
use crate::v5::author::update::UpdateAuthorBuilder;
use crate::HttpClientRef;

/// Author endpoint handler builder.
#[derive(Debug)]
pub struct AuthorBuilder {
    http_client: HttpClientRef,
}

impl AuthorBuilder {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }

    /// Search for authors.
    ///
    /// <https://api.mangadex.org/swagger.html#/Author/get-author>
    pub fn list(&self) -> ListAuthorBuilder {
        ListAuthorBuilder::default().http_client(self.http_client.clone())
    }

    /// Search for authors.
    ///
    /// <https://api.mangadex.org/swagger.html#/Author/get-author>
    ///
    /// This is an alias for the [`Self::list()`] function.
    pub fn search(&self) -> ListAuthorBuilder {
        self.list()
    }

    /// View a single author.
    ///
    /// <https://api.mangadex.org/swagger.html#/Author/get-author-id>
    pub fn get(&self) -> GetAuthorBuilder {
        GetAuthorBuilder::default().http_client(self.http_client.clone())
    }

    /// View a single author.
    ///
    /// <https://api.mangadex.org/swagger.html#/Author/get-author-id>
    ///
    /// This is an alias for [`Self::get()`] to maintain backwards-compatibility.
    pub fn view(&self) -> GetAuthorBuilder {
        self.get()
    }

    /// Create an author.
    ///
    /// <https://api.mangadex.org/swagger.html#/Author-author>
    pub fn create(&self) -> CreateAuthorBuilder {
        CreateAuthorBuilder::default().http_client(self.http_client.clone())
    }

    /// Update an author.
    ///
    /// <https://api.mangadex.org/swagger.html#/Author/put-author-id>
    pub fn update(&self) -> UpdateAuthorBuilder {
        UpdateAuthorBuilder::default().http_client(self.http_client.clone())
    }

    /// Delete an author.
    ///
    /// <https://api.mangadex.org/swagger.html#/Author/delete-author-id>
    pub fn delete(&self) -> DeleteAuthorBuilder {
        DeleteAuthorBuilder::default().http_client(self.http_client.clone())
    }
}
