//! Rating endpoint handler.
//!
//! <https://api.mangadex.org/swagger.html#/Rating/Rating>

#[cfg(not(feature = "deserializable-endpoint"))]
mod create_update_for_manga;
#[cfg(not(feature = "deserializable-endpoint"))]
mod delete_for_manga;
#[cfg(not(feature = "deserializable-endpoint"))]
mod get_your_manga_ratings;

#[cfg(feature = "deserializable-endpoint")]
pub mod create_update_for_manga;
#[cfg(feature = "deserializable-endpoint")]
pub mod delete_for_manga;
#[cfg(feature = "deserializable-endpoint")]
pub mod get_your_manga_ratings;

use crate::v5::rating::create_update_for_manga::CreateUpdateMangaRatingBuilder;
use crate::v5::rating::delete_for_manga::DeleteMangaRatingBuilder;
use crate::v5::rating::get_your_manga_ratings::GetYourMangaRatingsBuilder;
use crate::HttpClientRef;

/// Rating endpoint handler builder.
#[derive(Clone, Debug)]
pub struct RatingBuilder {
    http_client: HttpClientRef,
}

impl RatingBuilder {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }

    /// Remove a Manga rating for the authenticated user.
    ///
    /// <https://api.mangadex.org/swagger.html#/Rating/delete-rating-manga-id>
    pub fn delete_for_manga(&self) -> DeleteMangaRatingBuilder {
        DeleteMangaRatingBuilder::default().http_client(self.http_client.clone())
    }

    /// Get Manga ratings for the authenticated user.
    ///
    /// <https://api.mangadex.org/swagger.html#/Rating/get-rating>
    pub fn get_your_manga_ratings(&self) -> GetYourMangaRatingsBuilder {
        GetYourMangaRatingsBuilder::default().http_client(self.http_client.clone())
    }

    /// Create or update a Manga rating for the authenticated user.
    ///
    /// <https://api.mangadex.org/swagger.html#/Rating/post-rating-manga-id>
    pub fn upsert_for_manga(&self) -> CreateUpdateMangaRatingBuilder {
        CreateUpdateMangaRatingBuilder::default().http_client(self.http_client.clone())
    }
}
