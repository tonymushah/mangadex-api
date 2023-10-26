pub mod feed;
pub mod get;
pub mod id;

use crate::HttpClientRef;

use uuid::Uuid;

use feed::FeedEndpoint;
use get::FollowedMangaBuilder;
use id::IdEndpoint;

create_endpoint_node! {
    #[name] MangaEndpoint MangaEndpointMethods,
    #[agrs] {
        http_client: HttpClientRef,
    },
    #[methods] {
        feed() -> FeedEndpoint;
        get() -> FollowedMangaBuilder;
        id(id: Uuid,) -> IdEndpoint;
    }
}

impl MangaEndpointMethods for MangaEndpoint {
    fn feed(&self) -> FeedEndpoint {
        FeedEndpoint::new(self.http_client.clone())
    }

    fn get(&self) -> FollowedMangaBuilder {
        FollowedMangaBuilder::default().http_client(self.http_client.clone())
    }

    fn id(&self, id: Uuid) -> IdEndpoint {
        IdEndpoint::new(self.http_client.clone(), id)
    }
}
