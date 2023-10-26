pub mod get;

use crate::HttpClientRef;

use uuid::Uuid;

use self::get::HaveBookMarkedCustomListBuilder;

create_endpoint_node! {
    #[name] IdEndpoint IdEndpointMethods,
    #[args] {
        http_client: HttpClientRef,
        id: Uuid,
    },
    #[methods] {
        get() -> HaveBookMarkedCustomListBuilder;
    }
}

impl IdEndpointMethods for IdEndpoint {
    fn get(&self) -> HaveBookMarkedCustomListBuilder {
        HaveBookMarkedCustomListBuilder::default()
            .list_id(self.id)
            .http_client(self.http_client.clone())
    }
}
