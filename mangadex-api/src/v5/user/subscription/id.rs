pub mod get;

use crate::HttpClientRef;
use uuid::Uuid;

use get::IsSubscribedToCustomListBuilder;

create_endpoint_node! {
    #[name] IdEndpoint IdEndpointMethods,
    #[args] {
        http_client: HttpClientRef,
        id: Uuid,
    },
    #[methods] {
        get() -> IsSubscribedToCustomListBuilder;
    }
}

impl IdEndpointMethods for IdEndpoint {
    fn get(&self) -> IsSubscribedToCustomListBuilder {
        IsSubscribedToCustomListBuilder::default()
            .list_id(self.id)
            .http_client(self.http_client.clone())
    }
}
