pub mod get;

use crate::HttpClientRef;

use uuid::Uuid;

use self::get::IsBookmarkingGroupBuilder;

create_endpoint_node! {
    #[name] IdEndpoint IdEndpointMethods,
    #[args] {
        http_client: HttpClientRef,
        id: Uuid,
    },
    #[methods] {
        get() -> IsBookmarkingGroupBuilder;
    }
}

impl IdEndpointMethods for IdEndpoint {
    fn get(&self) -> IsBookmarkingGroupBuilder {
        IsBookmarkingGroupBuilder::default()
            .group_id(self.id)
            .http_client(self.http_client.clone())
    }
}
