pub mod api_client;
pub mod at_home;
pub mod auth;
pub mod author;
pub mod captcha;
pub mod chapter;
pub mod cover;
pub mod custom_list;
pub mod feed;
pub mod forums;
pub mod legacy;
pub mod manga;
cfg_oauth! {
    pub mod oauth;
}
pub mod ping;
pub mod rating;
pub mod report;
pub mod scanlation_group;
pub mod search;
pub mod settings;
pub mod statistics;
pub mod upload;
pub mod user;

use crate::Result;
pub use mangadex_api_schema::v5 as schema;
use mangadex_api_schema::v5::oauth::ClientInfo;
pub(crate) use mangadex_api_schema::v5::AuthTokens;

use reqwest::Client;

use std::sync::Arc;
use tokio::sync::RwLock;

#[cfg(feature = "utils")]
use crate::utils::download::DownloadBuilder;
use crate::v5::api_client::ApiClientEndpoint;
use crate::v5::at_home::AtHomeBuilder;
use crate::v5::auth::AuthBuilder;
use crate::v5::author::AuthorBuilder;
use crate::v5::captcha::CaptchaBuilder;
use crate::v5::chapter::ChapterBuilder;
use crate::v5::cover::CoverBuilder;
use crate::v5::custom_list::CustomListBuilder;
use crate::v5::feed::FeedBuilder;
use crate::v5::forums::ForumsEndpoint;
use crate::v5::legacy::LegacyBuilder;
use crate::v5::manga::MangaBuilder;
#[cfg(feature = "oauth")]
use crate::v5::oauth::OAuthBuider;
use crate::v5::ping::PingEndpointBuilder;
use crate::v5::rating::RatingBuilder;
use crate::v5::report::ReportBuilder;
use crate::v5::scanlation_group::ScanlationGroupBuilder;
use crate::v5::search::SearchBuilder;
use crate::v5::settings::SettingsBuilder;
use crate::v5::statistics::StatisticsBuilder;
use crate::v5::upload::UploadBuilder;
use crate::v5::user::UserBuilder;
use crate::HttpClient;
use crate::HttpClientRef;

/// API client to make requests to the MangaDex v5 API.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct MangaDexClient {
    pub http_client: HttpClientRef,
}

impl Default for MangaDexClient {
    /// Create a new `MangaDexClient` with the default [`reqwest::Client`](https://docs.rs/reqwest/latest/reqwest/struct.Client.html) settings.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use reqwest::Client;
    ///
    /// use mangadex_api::v5::MangaDexClient;
    ///
    /// # async fn run() -> Result<(), reqwest::Error> {
    /// let client = MangaDexClient::default();
    /// # Ok(())
    /// # }
    /// ```
    fn default() -> Self {
        Self::new_with_http_client(HttpClient::default())
    }
}

impl MangaDexClient {
    /// Create a new `MangaDexClient` with a custom [`reqwest::Client`](https://docs.rs/reqwest/latest/reqwest/struct.Client.html).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use reqwest::Client;
    ///
    /// use mangadex_api::v5::MangaDexClient;
    ///
    /// # async fn run() -> Result<(), reqwest::Error> {
    /// let reqwest_client = Client::builder()
    ///     .timeout(std::time::Duration::from_secs(10))
    ///     .build()?;
    ///
    /// let client = MangaDexClient::new(reqwest_client);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(client: Client) -> Self {
        Self::new_with_http_client_ref(create_ref_counted_http_client(HttpClient::new(client)))
    }

    /// Create a new `MangaDexClient` with a custom client reference
    pub fn new_with_http_client_ref(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }
    /// Create a new `MangaDexClient` with a custom [`HttpClient`].
    ///
    /// In most cases, providing a custom [`HttpClient`] isn't necessary.
    /// This function is primarily useful for mock testing but is available for anyone that needs to
    /// change the base URL if it changes due to an unforeseen event.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use reqwest::Client;
    /// use url::Url;
    ///
    /// use mangadex_api::v5::MangaDexClient;
    /// use mangadex_api::HttpClient;
    ///
    /// # async fn run() -> anyhow::Result<()> {
    /// let reqwest_client = Client::builder()
    ///     .timeout(std::time::Duration::from_secs(10))
    ///     .build()?;
    ///
    /// let http_client = HttpClient::builder()
    ///     .client(reqwest_client)
    ///     .base_url(Url::parse("127.0.0.1:8080")?)
    ///     .build()?;
    ///
    /// let client = MangaDexClient::new_with_http_client(http_client);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new_with_http_client(http_client: HttpClient) -> Self {
        Self::new_with_http_client_ref(create_ref_counted_http_client(http_client))
    }

    /// Return the Reqwest `Client`.
    ///
    /// This can be used to create manual HTTP requests.
    ///
    /// Using this is generally not advised as it can provide mutable access to the [`HttpClient`].
    pub fn get_http_client(&self) -> HttpClientRef {
        self.http_client.clone()
    }

    pub async fn set_auth_tokens(&self, auth_tokens: &AuthTokens) -> Result<()> {
        let mut client = self.http_client.write().await;

        client.set_auth_tokens(auth_tokens);
        Ok(())
    }

    pub async fn clear_auth_tokens(&self) -> Result<()> {
        let mut client = self.http_client.write().await;

        client.clear_auth_tokens();
        Ok(())
    }
    pub async fn get_auth_tokens(&self) -> Result<AuthTokens> {
        let client = &self.http_client.read().await;
        client
            .get_tokens()
            .cloned()
            .ok_or(crate::error::Error::MissingTokens)
    }

    pub async fn set_captcha<A: Into<String>>(&self, captcha: A) -> Result<()> {
        let mut client = self.http_client.write().await;
        client.set_captcha(captcha);
        Ok(())
    }
    pub async fn get_captcha(&self) -> Result<String> {
        let client = &self.http_client.read().await;
        client
            .get_captcha()
            .cloned()
            .ok_or(crate::error::Error::MissingCaptcha)
    }
    pub async fn clear_captcha(&self) -> Result<()> {
        let mut client = self.http_client.write().await;
        client.clear_captcha();
        Ok(())
    }

    cfg_oauth! {
        pub async fn get_client_info(&self) -> Result<ClientInfo> {
            let client = &self.http_client.read().await;
            client
                .get_client_info()
                .cloned()
                .ok_or(crate::error::Error::MissingClientInfo)
        }
    }
    /// Get a builder for handling the At-Home endpoints.
    ///
    /// <https://api.mangadex.org/swagger.html#/AtHome>
    pub fn at_home(&self) -> AtHomeBuilder {
        AtHomeBuilder::new(self.http_client.clone())
    }

    /// Get a builder for handling the authentication endpoints.
    ///
    /// This builder is deprecated
    ///
    /// <https://api.mangadex.org/docs/redoc.html#tag/Authentication>
    pub fn auth(&self) -> AuthBuilder {
        AuthBuilder::new(self.http_client.clone())
    }

    /// Get a builder for handling the author endpoints.
    ///
    /// <https://api.mangadex.org/swagger.html#/Author>
    pub fn author(&self) -> AuthorBuilder {
        AuthorBuilder::new(self.http_client.clone())
    }

    /// Get a builder for handling the captcha endpoints.
    ///
    /// <https://api.mangadex.org/swagger.html#/Captcha>
    pub fn captcha(&self) -> CaptchaBuilder {
        CaptchaBuilder::new(self.http_client.clone())
    }

    /// Get a builder for handling the chapter endpoints.
    ///
    /// <https://api.mangadex.org/swagger.html#/Chapter>
    pub fn chapter(&self) -> ChapterBuilder {
        ChapterBuilder::new(self.http_client.clone())
    }

    pub fn client(&self) -> ApiClientEndpoint {
        ApiClientEndpoint::new(self.http_client.clone())
    }

    /// Get a builder for handling manga volume cover art endpoints.
    ///
    /// <https://api.mangadex.org/swagger.html#/Cover>
    pub fn cover(&self) -> CoverBuilder {
        CoverBuilder::new(self.http_client.clone())
    }

    /// Get a builder for handling the custom list endpoints.
    ///
    /// <https://api.mangadex.org/swagger.html#/CustomList>
    pub fn custom_list(&self) -> CustomListBuilder {
        CustomListBuilder::new(self.http_client.clone())
    }

    /// Get a builder for handling the feed endpoints.
    ///
    /// <https://api.mangadex.org/swagger.html#/Feed>
    pub fn feed(&self) -> FeedBuilder {
        FeedBuilder::new(self.http_client.clone())
    }

    /// Get a builder for handling the infrastructure endpoints.
    ///
    /// <https://api.mangadex.org/swagger.html#/Infrastructure>
    pub fn ping(&self) -> PingEndpointBuilder {
        PingEndpointBuilder::new(self.http_client.clone())
    }

    /// Get a builder for handling the legacy endpoints.
    ///
    /// <https://api.mangadex.org/swagger.html#/Legacy>
    pub fn legacy(&self) -> LegacyBuilder {
        LegacyBuilder::new(self.http_client.clone())
    }

    /// Get a builder for handling the manga endpoints.
    ///
    /// <https://api.mangadex.org/swagger.html#/Manga>
    pub fn manga(&self) -> MangaBuilder {
        MangaBuilder::new(self.http_client.clone())
    }

    /// Get a builder for handling the rating endpoints.
    ///
    /// <https://api.mangadex.org/swagger.html#/Rating>
    pub fn rating(&self) -> RatingBuilder {
        RatingBuilder::new(self.http_client.clone())
    }

    /// Get a builder for handling the report endpoints.
    ///
    /// <https://api.mangadex.org/swagger.html#/Report>
    pub fn report(&self) -> ReportBuilder {
        ReportBuilder::new(self.http_client.clone())
    }

    /// Get a builder for handling the scanlation group endpoints.
    ///
    /// <https://api.mangadex.org/swagger.html#/ScanlationGroup>
    pub fn scanlation_group(&self) -> ScanlationGroupBuilder {
        ScanlationGroupBuilder::new(self.http_client.clone())
    }

    /// Get a builder for handling the search endpoints.
    ///
    /// This is a convenience builder that aggregates search endpoints from various categories.
    pub fn search(&self) -> SearchBuilder {
        SearchBuilder::new(self.http_client.clone())
    }

    /// Get a builder for handling the settings endpoints.
    ///
    /// <https://api.mangadex.org/swagger.html#/Settings>
    // Not public yet as the settings endpoints are not stable as of MangaDex API v5.4.9.
    pub fn settings(&self) -> SettingsBuilder {
        SettingsBuilder::new(self.http_client.clone())
    }

    /// Get a builder for handling the statistics endpoints.
    ///
    /// <https://api.mangadex.org/swagger.html#/Statistics>
    pub fn statistics(&self) -> StatisticsBuilder {
        StatisticsBuilder::new(self.http_client.clone())
    }

    /// Get a builder for handling uploads.
    ///
    /// <https://api.mangadex.org/swagger.html#/Upload>
    pub fn upload(&self) -> UploadBuilder {
        UploadBuilder::new(self.http_client.clone())
    }

    /// Get a builder for handling the user endpoints.
    ///
    /// <https://api.mangadex.org/swagger.html#/User>
    pub fn user(&self) -> UserBuilder {
        UserBuilder::new(self.http_client.clone())
    }

    /// This is an api client for
    /// `api.mangadex.dev`
    pub fn api_dev_client() -> Self {
        Self::new_with_http_client(HttpClient::api_dev_client())
    }
    cfg_utils! {
        pub fn download(&self) -> DownloadBuilder {
            DownloadBuilder::new(self.http_client.clone())
        }
    }

    pub fn forums(&self) -> ForumsEndpoint {
        ForumsEndpoint::new(self.http_client.clone())
    }

    cfg_oauth! {
        pub fn oauth(&self) -> OAuthBuider {
            OAuthBuider::new(self.http_client.clone())
        }
    }
    cfg_oauth! {
        pub async fn set_client_info(&self, client_info: &ClientInfo) -> Result<()> {
            let mut client = self.http_client.write().await;
            client.set_client_info(client_info);
            Ok(())
        }
    }
    cfg_oauth! {
        pub async fn clear_client_info(&self) -> Result<()> {
            let mut client = self.http_client.write().await;
            client.clear_client_info();
            Ok(())
        }
    }
    pub async fn get_reqwest_client(&self) -> reqwest::Client {
        self.get_http_client().read().await.client.clone()
    }
}

/// Create a new reference counted `HttpClient`.
fn create_ref_counted_http_client(http_client: HttpClient) -> HttpClientRef {
    Arc::new(RwLock::new(http_client))
}
