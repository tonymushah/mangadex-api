pub mod post;

use crate::HttpClientRef;

use post::UpdateUserEmailBuilder;

create_endpoint_node! {
    #[name] EmailEndpoint EmailEndpointMethods,
    #[args] {
        http_client: HttpClientRef,
    },
    #[methods] {
        post() -> UpdateUserEmailBuilder;
    }
}

impl EmailEndpointMethods for EmailEndpoint {
    fn post(&self) -> UpdateUserEmailBuilder {
        UpdateUserEmailBuilder::default().http_client(self.http_client.clone())
    }
}
