pub mod get;

use crate::HttpClientRef;

use get::GetMyUserDetailsBuilder;

create_endpoint_node! {
    #[name] MeEndpoint MeEndpointMethods,
    #[args] {
        http_client: HttpClientRef,
    },
    #[methods] {
        get() -> GetMyUserDetailsBuilder;
    }
}

impl MeEndpointMethods for MeEndpoint {
    fn get(&self) -> GetMyUserDetailsBuilder {
        GetMyUserDetailsBuilder::default().http_client(self.http_client.clone())
    }
}
