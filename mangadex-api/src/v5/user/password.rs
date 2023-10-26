pub mod post;

use crate::HttpClientRef;

use post::UpdateUserPasswordBuilder;

create_endpoint_node! {
    #[name] PasswordEndpoint PasswordEndpointMethods,
    #[args] {
        http_client: HttpClientRef,
    },
    #[methods] {
        post() -> UpdateUserPasswordBuilder;
    }
}

impl PasswordEndpointMethods for PasswordEndpoint {
    fn post(&self) -> UpdateUserPasswordBuilder {
        UpdateUserPasswordBuilder::default().http_client(self.http_client.clone())
    }
}
