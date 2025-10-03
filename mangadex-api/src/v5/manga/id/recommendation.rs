pub mod get;

use crate::HttpClientRef;

use uuid::Uuid;

use get::GetMangaRecommendationsBuilder;

#[derive(Debug)]
pub struct MangaRecommendationEndpoint {
    http_client: HttpClientRef,
    id: Uuid,
}

impl MangaRecommendationEndpoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef, id: Uuid) -> Self {
        Self { http_client, id }
    }
    pub fn get(&self) -> GetMangaRecommendationsBuilder {
        GetMangaRecommendationsBuilder::default()
            .manga_id(self.id)
            .http_client(self.http_client.clone())
    }
}
