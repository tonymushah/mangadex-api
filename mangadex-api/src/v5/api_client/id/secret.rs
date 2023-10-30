pub mod get;
pub mod post;

use crate::HttpClientRef;
use uuid::Uuid;

use get::GetClientSecretBuilder;
use post::RegenerateClientSecretBuilder;

create_endpoint_node! {
    #[name] SecretEndpoint SecretEndpointMethods,
    #[args] {
        http_client: HttpClientRef,
        id: Uuid,
    },
    #[methods] {
        get() -> GetClientSecretBuilder;
        post() -> RegenerateClientSecretBuilder;
    }
}

impl SecretEndpointMethods for SecretEndpoint {
    fn get(&self) -> GetClientSecretBuilder {
        GetClientSecretBuilder::default()
            .client_id(<&Self as Into<Uuid>>::into(self))
            .http_client(<&Self as Into<HttpClientRef>>::into(self))
    }

    fn post(&self) -> RegenerateClientSecretBuilder {
        RegenerateClientSecretBuilder::default()
            .client_id(<&Self as Into<Uuid>>::into(self))
            .http_client(<&Self as Into<HttpClientRef>>::into(self))
    }
}
