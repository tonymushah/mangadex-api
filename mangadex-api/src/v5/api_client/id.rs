pub mod delete;
pub mod get;
pub mod post;
pub mod secret;

use crate::HttpClientRef;
use uuid::Uuid;

use delete::DeleteClientBuilder;
use get::GetClientBuilder;
use post::EditClientBuilder;

create_endpoint_node! {
    #[name] IdEndpoint IdEndpointMethods,
    #[args] {
        http_client: HttpClientRef,
        id: Uuid,
    },
    #[methods] {
        get() -> GetClientBuilder;
        post() -> EditClientBuilder;
        delete() -> DeleteClientBuilder;
    }
}

impl IdEndpointMethods for IdEndpoint {
    fn get(&self) -> GetClientBuilder {
        GetClientBuilder::default()
            .http_client(<&Self as Into<HttpClientRef>>::into(self))
            .client_id(<&Self as Into<Uuid>>::into(self))
    }

    fn post(&self) -> EditClientBuilder {
        EditClientBuilder::default()
            .client_id(<&Self as Into<Uuid>>::into(self))
            .http_client(<&Self as Into<HttpClientRef>>::into(self))
    }

    fn delete(&self) -> DeleteClientBuilder {
        DeleteClientBuilder::default()
            .client_id(<&Self as Into<Uuid>>::into(self))
            .http_client(<&Self as Into<HttpClientRef>>::into(self))
    }
}
