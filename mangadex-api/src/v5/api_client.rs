pub mod get;
pub mod id;
pub mod post;

use get::ListClientsBuilder;

use crate::HttpClientRef;
use uuid::Uuid;

create_endpoint_node! {
    #[name] ApiClientEndpoint ApiClientEndpointMethods,
    #[args] {
        http_client: HttpClientRef,
    },
    #[methods] {
        get() -> ListClientsBuilder;
    }
}

impl ApiClientEndpointMethods for ApiClientEndpoint {
    fn get(&self) -> ListClientsBuilder {
        ListClientsBuilder::default().http_client(self.http_client.clone())
    }
}
