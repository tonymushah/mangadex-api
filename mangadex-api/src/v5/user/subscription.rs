pub mod id;

use crate::HttpClientRef;
use uuid::Uuid;

use id::IdEndpoint;

create_endpoint_node! {
    #[name] SubscriptionEndpoint SubscriptionEndpointMethods,
    #[args] {
        http_client: HttpClientRef,
    },
    #[methods] {
        id(id: Uuid,) -> IdEndpoint;
    }
}

impl SubscriptionEndpointMethods for SubscriptionEndpoint {
    fn id(&self, id: Uuid) -> IdEndpoint {
        IdEndpoint::new(self.http_client.clone(), id)
    }
}
