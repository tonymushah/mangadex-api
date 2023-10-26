pub mod group;
pub mod list;
pub mod user;

use crate::HttpClientRef;
use group::GroupEndpoint;
use list::ListEndpoint;
use user::UserEndpoint;

create_endpoint_node! {
    #[name] BookmarksEndpoint BookmarksEndpointMethods,
    #[args] {
        http_client: HttpClientRef,
    },
    #[methods] {
        group() -> GroupEndpoint;
        list() -> ListEndpoint;
        user() -> UserEndpoint;
    }
}

impl BookmarksEndpointMethods for BookmarksEndpoint {
    fn group(&self) -> GroupEndpoint {
        GroupEndpoint::new(self.http_client.clone())
    }

    fn list(&self) -> ListEndpoint {
        ListEndpoint::new(self.http_client.clone())
    }

    fn user(&self) -> UserEndpoint {
        UserEndpoint::new(self.http_client.clone())
    }
}
