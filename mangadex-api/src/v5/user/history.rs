pub mod get;

use crate::HttpClientRef;
use get::GetUserHistoryBuilder;

create_endpoint_node! {
    #[name] HistoryEndpoint HistoryEndpointMethods,
    #[args] {
        http_client: HttpClientRef,
    },
    #[methods] {
        get() -> GetUserHistoryBuilder;
    }
}

impl HistoryEndpointMethods for HistoryEndpoint {
    fn get(&self) -> GetUserHistoryBuilder {
        GetUserHistoryBuilder::default().http_client(self.http_client.clone())
    }
}
