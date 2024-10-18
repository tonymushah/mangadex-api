//! Feed endpoint handler.
//!
//! <https://api.mangadex.org/swagger.html#/Feed>

use uuid::Uuid;

use crate::v5::custom_list::id::feed::get::CustomListMangaFeedBuilder;
use crate::v5::user::follows::manga::get::FollowedMangaBuilder;
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
    /// <https://api.mangadex.org/docs/swagger.html#/Feed/get-user-follows-manga-feed>
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mangadex_api::v5::MangaDexClient;
    /// use mangadex_api_types::{Password, Username};
    ///
    /// # async fn run() -> anyhow::Result<()> {
    /// let client = MangaDexClient::default();
    ///
    /// let _login_res = client
    ///     .oauth()
    ///     .login()
    ///     .username(Username::parse("myusername")?)
    ///     .password(Password::parse("hunter23")?)
    ///     .send()
    ///     .await?;
    ///
    /// let res = client
    ///     .feed()
    ///     .followed_manga()
    ///     .limit(1_u32)
    ///     .send()
    ///     .await?;
    ///
    /// println!("results: {:?}", res);
    /// # Ok(())
    /// # }
    /// ```
    pub fn followed_manga(&self) -> FollowedMangaBuilder {
        FollowedMangaBuilder::default().http_client(self.http_client.clone())
    }

    /// Get the manga feed for a given custom list.
    ///
    /// <https://api.mangadex.org/docs/swagger.html#/Feed/get-list-id-feed>
    ///
    /// Alias to [`MangaDexClient::custom_list().id(uuid::Uuid).feed().get()`](crate::v5::custom_list::id::feed::get::CustomListMangaFeedBuilder).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mangadex_api::v5::MangaDexClient;
    /// use uuid::Uuid;
    ///
    /// # async fn run() -> anyhow::Result<()> {
    /// let client = MangaDexClient::default();
    ///
    /// let res = client
    ///     .feed()
    ///     .custom_list_manga(Uuid::new_v4())
    ///     .limit(1_u32)
    ///     .send()
    ///     .await?;
    ///
    /// println!("results: {:?}", res);
    /// # Ok(())
    /// # }
    /// ```
    pub fn custom_list_manga(&self, list_id: Uuid) -> CustomListMangaFeedBuilder {
        CustomListMangaFeedBuilder::default()
            .list_id(list_id)
            .http_client(self.http_client.clone())
    }
}
