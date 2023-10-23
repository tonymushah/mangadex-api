pub mod get;

use crate::HttpClientRef;

use uuid::Uuid;

use self::get::HaveBookMarkedUserBuilder;

create_endpoint_node! {
    #[name] IdEndpoint IdEndpointMethods,
    #[args] {
        http_client: HttpClientRef,
        id: Uuid,
    },
    #[methods] {
        get() -> HaveBookMarkedUserBuilder;
    }
}

impl IdEndpointMethods for IdEndpoint {
    fn get(&self) -> HaveBookMarkedUserBuilder {
        HaveBookMarkedUserBuilder::default()
            .user_id(self.id)
            .http_client(self.http_client.clone())
    }
}
