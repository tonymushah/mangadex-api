pub mod get;

use crate::HttpClientRef;

use get::MyDefaultCustomListsBuilder;

create_endpoint_node! {
    #[name] DefaultEndpoint DefaultEndpointMethods,
    #[args] {
        http_client: HttpClientRef,
    },
    #[methods] {
        get() -> MyDefaultCustomListsBuilder;
    }
}

impl DefaultEndpointMethods for DefaultEndpoint {
    fn get(&self) -> MyDefaultCustomListsBuilder {
        MyDefaultCustomListsBuilder::default().http_client(self.http_client.clone())
    }
}
