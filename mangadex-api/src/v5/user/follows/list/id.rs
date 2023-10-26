pub mod get;

use get::IsFollowingCustomListBuilder;

use crate::HttpClientRef;
use uuid::Uuid;

create_endpoint_node! {
    #[name] IdEndpoint IdEndpointMethods,
    #[args] {
        http_client: HttpClientRef,
        id: Uuid,
    },
    #[methods] {
        get() -> IsFollowingCustomListBuilder;
    }
}

impl IdEndpointMethods for IdEndpoint {
    fn get(&self) -> IsFollowingCustomListBuilder {
        IsFollowingCustomListBuilder::default()
            .list_id(self.id)
            .http_client(self.http_client.clone())
    }
}
