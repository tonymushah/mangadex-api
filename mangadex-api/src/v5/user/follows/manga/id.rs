pub mod get;

use crate::HttpClientRef;
use get::IsFollowingMangaBuilder;
use uuid::Uuid;

create_endpoint_node! {
    #[name] IdEndpoint IdEndpointMethods,
    #[args] {
        http_client : HttpClientRef,
        id: Uuid,
    },
    #[methods] {
        get() -> IsFollowingMangaBuilder;
    }
}

impl IdEndpointMethods for IdEndpoint {
    fn get(&self) -> IsFollowingMangaBuilder {
        IsFollowingMangaBuilder::default()
            .manga_id(self.id)
            .http_client(self.http_client.clone())
    }
}
