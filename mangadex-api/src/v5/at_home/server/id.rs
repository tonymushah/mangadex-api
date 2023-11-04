pub mod get;

use crate::HttpClientRef;
use uuid::Uuid;

use get::GetAtHomeServerBuilder;

create_endpoint_node! {
    #[name] IdEndpoint IdEndpointMethods,
    #[args] {
        http_client: HttpClientRef,
        id: Uuid,
    },
    #[methods] {
        get() -> GetAtHomeServerBuilder;
    }
}

impl IdEndpointMethods for IdEndpoint {
    fn get(&self) -> GetAtHomeServerBuilder {
        GetAtHomeServerBuilder::default()
            .http_client(<&Self as Into<HttpClientRef>>::into(self))
            .chapter_id(<&Self as Into<Uuid>>::into(self))
    }
}
