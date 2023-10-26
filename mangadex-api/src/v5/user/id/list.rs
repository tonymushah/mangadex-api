pub mod get;

use crate::HttpClientRef;
use uuid::Uuid;

use get::UserCustomListsBuilder;

create_endpoint_node! {
    #[name] ListEndpoint ListEndpointMethods,
    #[args] {
        http_client: HttpClientRef,
        id: Uuid,
    },
    #[methods] {
        get() -> UserCustomListsBuilder;
    }
}

impl ListEndpointMethods for ListEndpoint {
    fn get(&self) -> UserCustomListsBuilder {
        UserCustomListsBuilder::default()
            .user_id(self.id)
            .http_client(self.http_client.clone())
    }
}
