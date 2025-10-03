pub mod aggregate;
pub mod delete;
pub mod feed;
pub mod follow;
pub mod get;
pub mod list;
pub mod put;
pub mod read;
pub mod recommendation;
pub mod status;

use crate::{v5::manga::id::recommendation::MangaRecommendationEndpoint, HttpClientRef};

use uuid::Uuid;

use aggregate::AggregateEndpoint;
use delete::DeleteMangaBuilder;
use feed::FeedEndpoint;
use follow::FollowEndpoint;
use get::GetMangaBuilder;
use list::ListEndpoint;
use put::UpdateMangaBuilder;
use read::ReadEndpoint;
use status::StatusEndpoint;

#[derive(Debug)]
pub struct IdEndpoint {
    http_client: HttpClientRef,
    id: Uuid,
}

impl IdEndpoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef, id: Uuid) -> Self {
        Self { http_client, id }
    }
    pub fn aggregate(&self) -> AggregateEndpoint {
        AggregateEndpoint::new(self.http_client.clone(), self.id)
    }
    pub fn delete(&self) -> DeleteMangaBuilder {
        DeleteMangaBuilder::default()
            .manga_id(self.id)
            .http_client(self.http_client.clone())
    }
    pub fn feed(&self) -> FeedEndpoint {
        FeedEndpoint::new(self.http_client.clone(), self.id)
    }
    pub fn follow(&self) -> FollowEndpoint {
        FollowEndpoint::new(self.http_client.clone(), self.id)
    }
    pub fn get(&self) -> GetMangaBuilder {
        GetMangaBuilder::default()
            .manga_id(self.id)
            .http_client(self.http_client.clone())
    }
    pub fn list(&self) -> ListEndpoint {
        ListEndpoint::new(self.http_client.clone(), self.id)
    }
    pub fn put(&self) -> UpdateMangaBuilder {
        UpdateMangaBuilder::default()
            .manga_id(self.id)
            .http_client(self.http_client.clone())
    }
    pub fn read(&self) -> ReadEndpoint {
        ReadEndpoint::new(self.http_client.clone(), self.id)
    }
    pub fn status(&self) -> StatusEndpoint {
        StatusEndpoint::new(self.http_client.clone(), self.id)
    }
    pub fn recommendation(&self) -> MangaRecommendationEndpoint {
        MangaRecommendationEndpoint::new(self.http_client.clone(), self.id)
    }
}
