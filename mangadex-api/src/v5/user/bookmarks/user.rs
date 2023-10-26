pub mod get;
pub mod id;

use get::BookmarkedUsersBuilder;
use id::IdEndpoint;
use uuid::Uuid;

use crate::HttpClientRef;

create_endpoint_node! {
    #[name] UserEndpoint UserEndpointMethods,
    #[args] {
        http_client: HttpClientRef,
    },
    #[methods] {
        get() -> BookmarkedUsersBuilder;
        id(id: Uuid, ) -> IdEndpoint;
    }
}

impl UserEndpointMethods for UserEndpoint {
    fn get(&self) -> BookmarkedUsersBuilder {
        BookmarkedUsersBuilder::default().http_client(self.http_client.clone())
    }

    fn id(&self, id: Uuid) -> IdEndpoint {
        IdEndpoint::new(self.http_client.clone(), id)
    }
}
