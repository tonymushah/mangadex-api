pub mod get;

use crate::HttpClientRef;
use get::IsFollowingGroupBuilder;
use uuid::Uuid;

create_endpoint_node! {
    #[name] IdEndpoint IdEndpointMethods,
    #[args] {
        http_client: HttpClientRef,
        id: Uuid,
    },
    #[methods] {
        get() -> IsFollowingGroupBuilder;
    }
}

impl IdEndpointMethods for IdEndpoint {
    fn get(&self) -> IsFollowingGroupBuilder {
        IsFollowingGroupBuilder::default()
            .group_id(self.id)
            .http_client(self.http_client.clone())
    }
}
