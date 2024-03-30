pub mod post;

use crate::HttpClientRef;

use post::CheckApprovalRequiredBuilder;

create_endpoint_node! {
    #[name] CheckApprovalRequiredEndpoint CheckApprovalRequiredEndpointMethods,
    #[args] {
        http_client: HttpClientRef,
    },
    #[methods] {
        post() -> CheckApprovalRequiredBuilder;
    }
}

impl CheckApprovalRequiredEndpointMethods for CheckApprovalRequiredEndpoint {
    fn post(&self) -> CheckApprovalRequiredBuilder {
        CheckApprovalRequiredBuilder::default().http_client(self.http_client.clone())
    }
}
