pub mod get;
pub mod id;

use crate::HttpClientRef;
use get::FollowedGroupsBuilder;
use id::IdEndpoint;

use uuid::Uuid;

create_endpoint_node! {
    #[name] GroupEndpoint GroupEndpointMethods,
    #[args] {
        http_client : HttpClientRef,
    },
    #[methods] {
        get() -> FollowedGroupsBuilder;
        id(id: Uuid,) -> IdEndpoint;
    }
}

impl GroupEndpointMethods for GroupEndpoint {
    fn get(&self) -> FollowedGroupsBuilder {
        FollowedGroupsBuilder::default().http_client(self.http_client.clone())
    }

    fn id(&self, id: Uuid) -> IdEndpoint {
        IdEndpoint::new(self.http_client.clone(), id)
    }
}
