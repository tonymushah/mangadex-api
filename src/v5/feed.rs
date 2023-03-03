//! Feed endpoint handler.
//!
//! <https://api.mangadex.org/swagger.html#/Feed>

use crate::v5::custom_list::manga_feed::CustomListMangaFeedBuilder;
use crate::v5::user::followed_manga_feed::GetFollowedMangaFeedBuilder;
use crate::HttpClientRef;

/// Feed endpoint handler builder.
#[derive(Debug)]
pub struct FeedBuilder {
    http_client: HttpClientRef,
}

impl FeedBuilder {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }

    /// Get the manga feed for the logged-in user.
    ///
    /// <https://api.mangadex.org/swagger.html#/Feed/get-user-follows-manga-feed>
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mangadex_api::v5::MangaDexClient;
    /// use mangadex_api::types::{Password, Username};
    ///
    /// # async fn run() -> anyhow::Result<()> {
    /// let client = MangaDexClient::default();
    ///
    /// let _login_res = client
    ///     .auth()
    ///     .login()
    ///     .username(Username::parse("myusername")?)
    ///     .password(Password::parse("hunter23")?)
    ///     .build()?
    ///     .send()
    ///     .await?;
    ///
    /// let res = client
    ///     .feed()
    ///     .followed_manga()
    ///     .limit(1_u32)
    ///     .build()?
    ///     .send()
    ///     .await?;
    ///
    /// println!("results: {:?}", res);
    /// # Ok(())
    /// # }
    /// ```
    pub fn followed_manga(&self) -> GetFollowedMangaFeedBuilder {
        GetFollowedMangaFeedBuilder::default().http_client(self.http_client.clone())
    }

    /// Get the manga feed for a given custom list.
    ///
    /// <https://api.mangadex.org/swagger.html#/Feed/get-list-id-feed>
    ///
    /// Alias to [`MangaDexClient::custom_list().manga_feed()`](crate::v5::custom_list::manga_feed).
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
    ///     .feed()
    ///     .custom_list_manga()
    ///     .limit(1_u32)
    ///     .build()?
    ///     .send()
    ///     .await?;
    ///
    /// println!("results: {:?}", res);
    /// # Ok(())
    /// # }
    /// ```
    pub fn custom_list_manga(&self) -> CustomListMangaFeedBuilder {
        CustomListMangaFeedBuilder::default().http_client(self.http_client.clone())
    }
}
