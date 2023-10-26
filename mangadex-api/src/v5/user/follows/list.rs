pub mod get;
pub mod id;

use crate::HttpClientRef;
use get::GetFollowedCustomListsBuilder;
use id::IdEndpoint;
use uuid::Uuid;

create_endpoint_node! {
    #[name] ListEndpoint ListEndpointMethods,
    #[args] {
        http_client: HttpClientRef,
    },
    #[methods] {
        get() -> GetFollowedCustomListsBuilder;
        id(id: Uuid,) -> IdEndpoint;
    }
}

impl ListEndpointMethods for ListEndpoint {
    fn get(&self) -> GetFollowedCustomListsBuilder {
        GetFollowedCustomListsBuilder::default().http_client(self.http_client.clone())
    }

    fn id(&self, id: Uuid) -> IdEndpoint {
        IdEndpoint::new(self.http_client.clone(), id)
    }
}
