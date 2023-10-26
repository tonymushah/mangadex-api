pub mod get;

use crate::HttpClientRef;
use get::HaveFollowedUserBuilder;
use uuid::Uuid;

create_endpoint_node! {
    #[name] IdEndpoint IdEndpointMethods,
    #[args] {
        http_client : HttpClientRef,
        id: Uuid,
    },
    #[methods] {
        get() -> HaveFollowedUserBuilder;
    }
}

impl IdEndpointMethods for IdEndpoint {
    fn get(&self) -> HaveFollowedUserBuilder {
        HaveFollowedUserBuilder::default()
            .user_id(self.id)
            .http_client(self.http_client.clone())
    }
}
