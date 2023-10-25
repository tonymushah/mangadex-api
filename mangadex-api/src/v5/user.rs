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

use bookmarks::BookmarksEndpoint;
use get::ListUserBuilder;

use delete::DeleteEndpoint;

create_endpoint_node! {
    #[name] UserBuilder UserBuilderMethods,
    #[args] {
        http_client: HttpClientRef,
    },
    #[methods] {
        bookmarks() -> BookmarksEndpoint;
        get() -> ListUserBuilder;
        delete() -> DeleteEndpoint;
    }
}

impl UserBuilderMethods for UserBuilder {
    fn bookmarks(&self) -> BookmarksEndpoint {
        BookmarksEndpoint::new(self.http_client.clone())
    }

    fn get(&self) -> ListUserBuilder {
        ListUserBuilder::default().http_client(self.http_client.clone())
    }

    fn delete(&self) -> DeleteEndpoint {
        DeleteEndpoint::new(self.http_client.clone())
    }
}
