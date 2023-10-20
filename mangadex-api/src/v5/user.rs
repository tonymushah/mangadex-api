//! User endpoint handler.
//!
//! <https://api.mangadex.org/swagger.html#/User>

pub mod bookmarks;
pub mod delete;
pub mod email;
pub mod follows;
pub mod get;
pub mod history;
pub mod id;
pub mod me;
pub mod password;
pub mod subscription;

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

}
