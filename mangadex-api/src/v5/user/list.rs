pub mod default;
pub mod get;

use crate::HttpClientRef;

use default::DefaultEndpoint;
use get::MyCustomListsBuilder;

create_endpoint_node! {
    #[name] ListEndpoint ListEndpointMethods,
    #[args] {
        http_client: HttpClientRef,
    },
    #[methods] {
        get() -> MyCustomListsBuilder;
        default() -> DefaultEndpoint;
    }
}

impl ListEndpointMethods for ListEndpoint {
    fn get(&self) -> MyCustomListsBuilder {
        MyCustomListsBuilder::default().http_client(self.http_client.clone())
    }
    fn default(&self) -> DefaultEndpoint {
        DefaultEndpoint::new(self.http_client.clone())
    }
}
