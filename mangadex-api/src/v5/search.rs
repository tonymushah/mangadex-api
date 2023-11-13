//! Search endpoint handler.
//!
//! This is a convenience builder for searching various categories.

use crate::v5::author::get::ListAuthorBuilder;
use crate::v5::chapter::get::ListChapterBuilder;
use crate::v5::cover::get::ListCoverBuilder;
use crate::v5::manga::get::ListMangaBuilder;
use crate::v5::scanlation_group::get::ListGroupBuilder;
use crate::HttpClientRef;

/// Search endpoint handler builder.
#[derive(Debug)]
pub struct SearchBuilder {
    http_client: HttpClientRef,
}

impl SearchBuilder {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }

    /// Search for authors.
    ///
    /// <https://api.mangadex.org/swagger.html#/Author/get-author>
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mangadex_api::v5::MangaDexClient;
    ///
    /// # async fn run() -> anyhow::Result<()> {
    /// let client = MangaDexClient::default();
    ///
    /// let res = client
    ///     .search()
    ///     .author()
    ///     .name("carlo zen")
    ///     .send()
    ///     .await?;
    ///
    /// println!("results: {:?}", res);
    /// # Ok(())
    /// # }
    /// ```
    pub fn author(&self) -> ListAuthorBuilder {
        ListAuthorBuilder::default().http_client(self.http_client.clone())
    }

    /// Search for chapters.
    ///
    /// <https://api.mangadex.org/swagger.html#/Chapter/get-chapter>
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mangadex_api::v5::MangaDexClient;
    ///
    /// # async fn run() -> anyhow::Result<()> {
    /// let client = MangaDexClient::default();
    ///
    /// let res = client
    ///     .search()
    ///     .chapter()
    ///     .title("summoning")
    ///     .send()
    ///     .await?;
    ///
    /// println!("results: {:?}", res);
    /// # Ok(())
    /// # }
    /// ```
    pub fn chapter(&self) -> ListChapterBuilder {
        ListChapterBuilder::default().http_client(self.http_client.clone())
    }

    /// Search for cover art.
    ///
    /// <https://api.mangadex.org/swagger.html#/Cover/get-cover>
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mangadex_api::v5::MangaDexClient;
    ///
    /// # async fn run() -> anyhow::Result<()> {
    /// let client = MangaDexClient::default();
    ///
    /// let res = client
    ///     .search()
    ///     .cover()
    ///     .send()
    ///     .await?;
    ///
    /// println!("results: {:?}", res);
    /// # Ok(())
    /// # }
    /// ```
    pub fn cover(&self) -> ListCoverBuilder {
        ListCoverBuilder::default().http_client(self.http_client.clone())
    }

    /// Search for manga.
    ///
    /// <https://api.mangadex.org/swagger.html#/Manga/get-search-manga>
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mangadex_api::v5::MangaDexClient;
    ///
    /// # async fn run() -> anyhow::Result<()> {
    /// let client = MangaDexClient::default();
    ///
    /// let res = client
    ///     .search()
    ///     .manga()
    ///     .title("official test manga")
    ///     .send()
    ///     .await?;
    ///
    /// println!("results: {:?}", res);
    /// # Ok(())
    /// # }
    /// ```
    pub fn manga(&self) -> ListMangaBuilder {
        ListMangaBuilder::default().http_client(self.http_client.clone())
    }

    /// Search for scanlation groups.
    ///
    /// <https://api.mangadex.org/swagger.html#/ScanlationGroup/get-search-group>
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mangadex_api::v5::MangaDexClient;
    ///
    /// # async fn run() -> anyhow::Result<()> {
    /// let client = MangaDexClient::default();
    ///
    /// let res = client
    ///     .search()
    ///     .scanlation_group()
    ///     .name("mangadex")
    ///     .send()
    ///     .await?;
    ///
    /// println!("results: {:?}", res);
    /// # Ok(())
    /// # }
    /// ```
    pub fn scanlation_group(&self) -> ListGroupBuilder {
        ListGroupBuilder::default().http_client(self.http_client.clone())
    }
}
