pub mod code;

use code::CodeEndpoint;

use crate::HttpClientRef;

use uuid::Uuid;

create_endpoint_node! {
    #[name] DeleteEndpoint DeleteEndpointMehods,
    #[args] {
        http_client: HttpClientRef,
    },
    #[methods] {
        code(code : Uuid,) -> CodeEndpoint;
    }
}

impl DeleteEndpointMehods for DeleteEndpoint {
    fn code(&self, code: Uuid) -> CodeEndpoint {
        CodeEndpoint::new(self.http_client.clone(), code)
    }
}
