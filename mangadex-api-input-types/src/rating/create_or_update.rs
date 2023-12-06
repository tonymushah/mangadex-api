#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::rating::manga_id::post::CreateUpdateMangaRatingBuilder, MangaDexClient};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
pub struct CreateUpdateRating {
    pub manga_id: Uuid,

    /// `[ 1 .. 10 ]`.
    ///
    /// Numbers below `1` will be set at `1` and numbers above `10` will be set at `10`.
    pub rating: u8,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<CreateUpdateRating> for CreateUpdateMangaRatingBuilder {
    fn from(value: CreateUpdateRating) -> Self {
        let mut builder = Self::default();
        builder.manga_id(value.manga_id);
        builder.rating(value.rating);
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl CreateUpdateRating {
    pub async fn send(self, client: &MangaDexClient) -> mangadex_api_types::error::Result<()> {
        <CreateUpdateMangaRatingBuilder as From<Self>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await?;
        Ok(())
    }
}
