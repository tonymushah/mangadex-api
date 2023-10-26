pub mod group;
pub mod list;
pub mod manga;
pub mod user;

use group::GroupEndpoint;
use list::ListEndpoint;
use manga::MangaEndpoint;
use user::UserEndpoint;

use crate::HttpClientRef;

create_endpoint_node! {
    #[name] FollowsEndpoint FollowsEndpointMethods,
    #[args] {
        http_client: HttpClientRef,
    },
    #[methods] {
        group() -> GroupEndpoint;
        list() -> ListEndpoint;
        manga() -> MangaEndpoint;
        user() -> UserEndpoint;
    }
}

impl FollowsEndpointMethods for FollowsEndpoint {
    fn group(&self) -> GroupEndpoint {
        GroupEndpoint::new(self.http_client.clone())
    }

    fn list(&self) -> ListEndpoint {
        ListEndpoint::new(self.http_client.clone())
    }

    fn manga(&self) -> MangaEndpoint {
        MangaEndpoint::new(self.http_client.clone())
    }

    fn user(&self) -> UserEndpoint {
        UserEndpoint::new(self.http_client.clone())
    }
}
