pub mod delete;
pub mod get;
pub mod post;
pub mod secret;

use crate::HttpClientRef;
use uuid::Uuid;

use get::GetClientBuilder;

create_endpoint_node! {
    #[name] IdEndpoint IdEndpointMethods,
    #[args] {
        http_client: HttpClientRef,
        id: Uuid,
    },
    #[methods] {
        get() -> GetClientBuilder;
    }
}

impl IdEndpointMethods for IdEndpoint {
    fn get(&self) -> GetClientBuilder {
        GetClientBuilder::default()
            .http_client(<&Self as Into<HttpClientRef>>::into(self))
            .client_id(<&Self as Into<Uuid>>::into(self))
    }
}
