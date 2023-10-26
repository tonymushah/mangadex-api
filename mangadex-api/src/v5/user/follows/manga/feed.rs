pub mod get;

use crate::HttpClientRef;

use get::GetFollowedMangaFeedBuilder;

create_endpoint_node! {
    #[name] FeedEndpoint FeedEndpointMethods,
    #[args] {
        http_client: HttpClientRef,
    },
    #[methods] {
        get() -> GetFollowedMangaFeedBuilder;
    }
}

impl FeedEndpointMethods for FeedEndpoint {
    fn get(&self) -> GetFollowedMangaFeedBuilder {
        GetFollowedMangaFeedBuilder::default().http_client(self.http_client.clone())
    }
}
