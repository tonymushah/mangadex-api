pub mod post;

use crate::HttpClientRef;
use post::ApproveUserDeletionBuilder;
use uuid::Uuid;

create_endpoint_node! {
    #[name] CodeEndpoint CodeEndpointMethods,
    #[args] {
        http_client: HttpClientRef,
        code: Uuid,
    },
    #[methods] {
        post() -> ApproveUserDeletionBuilder;
    }
}

impl CodeEndpointMethods for CodeEndpoint {
    fn post(&self) -> ApproveUserDeletionBuilder {
        ApproveUserDeletionBuilder::default()
            .code(self.code)
            .http_client(self.http_client.clone())
    }
}
