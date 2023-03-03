//! Statistics endpoint handler.
//!
//! <https://api.mangadex.org/swagger.html#/Statistics>

mod find_manga;
mod get_manga;

use crate::v5::statistics::find_manga::FindMangaStatisticsBuilder;
use crate::v5::statistics::get_manga::GetMangaStatisticsBuilder;
use crate::HttpClientRef;

/// Statistics endpoint handler builder.
#[derive(Clone, Debug)]
pub struct StatisticsBuilder {
    http_client: HttpClientRef,
}

impl StatisticsBuilder {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }

    /// Find statistics about given Manga.
    ///
    /// <https://api.mangadex.org/swagger.html#/Statistics/get-statistics-manga>
    ///
    /// This endpoint allows searching multiple Manga.
    pub fn find_manga(&self) -> FindMangaStatisticsBuilder {
        FindMangaStatisticsBuilder::default().http_client(self.http_client.clone())
    }

    /// Get statistics about a given Manga.
    ///
    /// <https://api.mangadex.org/swagger.html#/Statistics/get-statistics-manga-uuid>
    ///
    /// This endpoint fetches statistics for a single Manga.
    pub fn get_manga(&self) -> GetMangaStatisticsBuilder {
        GetMangaStatisticsBuilder::default().http_client(self.http_client.clone())
    }
}
