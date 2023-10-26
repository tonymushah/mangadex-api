pub mod delete;
pub mod post;

use delete::UnFollowUserBuilder;
use post::FollowUserBuilder;

use crate::HttpClientRef;
use uuid::Uuid;

create_endpoint_node! {
    #[name] FollowEndpoint FollowEndpointMethods,
    #[args] {
        http_client: HttpClientRef,
        id: Uuid,
    },
    #[methods] {
        delete() -> UnFollowUserBuilder;
        post() -> FollowUserBuilder;
    }
}

impl FollowEndpointMethods for FollowEndpoint {
    fn delete(&self) -> UnFollowUserBuilder {
        UnFollowUserBuilder::default()
            .user_id(self.id)
            .http_client(self.http_client.clone())
    }

    fn post(&self) -> FollowUserBuilder {
        FollowUserBuilder::default()
            .user_id(self.id)
            .http_client(self.http_client.clone())
    }
}
