pub mod delete;
pub mod post;

use delete::UnBookmarkUserBuilder;
use post::BookmarkUserBuilder;

use crate::HttpClientRef;
use uuid::Uuid;

create_endpoint_node! {
    #[name] BookmarkEndpoint BookmarkEndpointMethods,
    #[args] {
        http_client: HttpClientRef,
        id: Uuid,
    },
    #[methods] {
        delete() -> UnBookmarkUserBuilder;
        post() -> BookmarkUserBuilder;
    }
}

impl BookmarkEndpointMethods for BookmarkEndpoint {
    fn delete(&self) -> UnBookmarkUserBuilder {
        UnBookmarkUserBuilder::default()
            .user_id(self.id)
            .http_client(self.http_client.clone())
    }

    fn post(&self) -> BookmarkUserBuilder {
        BookmarkUserBuilder::default()
            .user_id(self.id)
            .http_client(self.http_client.clone())
    }
}
