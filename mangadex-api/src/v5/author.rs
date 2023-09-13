//! Author endpoint handler.
//!
//! <https://api.mangadex.org/swagger.html#/Author>

pub mod get;
pub mod id;
pub mod post;

use uuid::Uuid;

use crate::v5::author::get::ListAuthorBuilder;

use crate::HttpClientRef;

use self::post::CreateAuthorBuilder;

use self::id::IdEndpoint;

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

    // TODO add docs
    pub fn get(&self) -> ListAuthorBuilder {
        ListAuthorBuilder::default().http_client(self.http_client.clone())
    }
    /// Create an author.
    ///
    /// <https://api.mangadex.org/swagger.html#/Author-author>
    pub fn post(&self) -> CreateAuthorBuilder {
        CreateAuthorBuilder::default().http_client(self.http_client.clone())
    }

    // TODO add docs
    pub fn id(&self, id: Uuid) -> IdEndpoint {
        IdEndpoint::new(self.http_client.clone(), id)
    }
}
