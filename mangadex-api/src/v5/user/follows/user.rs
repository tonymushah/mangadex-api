pub mod get;
pub mod id;

use get::FollowedUsersBuilder;
use id::IdEndpoint;

use crate::HttpClientRef;
use uuid::Uuid;

create_endpoint_node! {
    #[name] UserEndpoint UserEndpointMethods,
    #[args] {
        http_client: HttpClientRef,
    },
    #[methods] {
        get() -> FollowedUsersBuilder;
        id(id: Uuid,) -> IdEndpoint;
    }
}

impl UserEndpointMethods for UserEndpoint {
    fn get(&self) -> FollowedUsersBuilder {
        FollowedUsersBuilder::default().http_client(self.http_client.clone())
    }

    fn id(&self, id: Uuid) -> IdEndpoint {
        IdEndpoint::new(self.http_client.clone(), id)
    }
}
