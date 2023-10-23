pub mod get;
pub mod id;

use get::BookmarkedGroupsBuilder;
use id::IdEndpoint;
use uuid::Uuid;

use crate::HttpClientRef;

create_endpoint_node! {
    #[name] GroupEndpoint GroupEndpointMethods,
    #[args] {
        http_client: HttpClientRef,
    },
    #[methods] {
        get() -> BookmarkedGroupsBuilder;
        id(id: Uuid, ) -> IdEndpoint;
    }
}

impl GroupEndpointMethods for GroupEndpoint {
    fn get(&self) -> BookmarkedGroupsBuilder {
        BookmarkedGroupsBuilder::default().http_client(self.http_client.clone())
    }

    fn id(&self, id: Uuid) -> IdEndpoint {
        IdEndpoint::new(self.http_client.clone(), id)
    }
}
