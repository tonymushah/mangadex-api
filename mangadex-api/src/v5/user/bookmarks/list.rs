pub mod get;
pub mod id;

use get::BookmarkedCustomListsBuilder;
use id::IdEndpoint;
use uuid::Uuid;

use crate::HttpClientRef;

create_endpoint_node! {
    #[name] ListEndpoint ListEndpointMethods,
    #[args] {
        http_client: HttpClientRef,
    },
    #[methods] {
        get() -> BookmarkedCustomListsBuilder;
        id(id: Uuid, ) -> IdEndpoint;
    }
}

impl ListEndpointMethods for ListEndpoint {
    fn get(&self) -> BookmarkedCustomListsBuilder {
        BookmarkedCustomListsBuilder::default().http_client(self.http_client.clone())
    }

    fn id(&self, id: Uuid) -> IdEndpoint {
        IdEndpoint::new(self.http_client.clone(), id)
    }
}
