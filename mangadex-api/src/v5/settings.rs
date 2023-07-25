//! Settings endpoint handler.
//!
//! <https://api.mangadex.org/swagger.html#/Settings>

#[cfg(not(feature = "deserializable-endpoint"))]
mod create_or_update_user_settings;
#[cfg(not(feature = "deserializable-endpoint"))]
mod create_template;
#[cfg(not(feature = "deserializable-endpoint"))]
mod get_latest_template;
#[cfg(not(feature = "deserializable-endpoint"))]
mod get_template_by_version_id;
#[cfg(not(feature = "deserializable-endpoint"))]
mod get_user_settings;

#[cfg(feature = "deserializable-endpoint")]
pub mod create_or_update_user_settings;
#[cfg(feature = "deserializable-endpoint")]
pub mod create_template;
#[cfg(feature = "deserializable-endpoint")]
pub mod get_latest_template;
#[cfg(feature = "deserializable-endpoint")]
pub mod get_template_by_version_id;
#[cfg(feature = "deserializable-endpoint")]
pub mod get_user_settings;

use crate::v5::settings::create_or_update_user_settings::CreateOrUpdateUserSettingsBuilder;
use crate::v5::settings::create_template::CreateSettingsTemplateBuilder;
use crate::v5::settings::get_latest_template::GetLatestSettingsTemplateBuilder;
use crate::v5::settings::get_template_by_version_id::GetSettingsTemplateByVersionIdBuilder;
use crate::v5::settings::get_user_settings::GetUserSettingsBuilder;
use crate::HttpClientRef;

/// Settings endpoint handler builder.
#[derive(Debug)]
pub struct SettingsBuilder {
    #[allow(unused)]
    http_client: HttpClientRef,
}

impl SettingsBuilder {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }

    /// Create or update a user's Settings template.
    ///
    /// <https://api.mangadex.org/swagger.html#/Settings/post-settings>
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use mangadex_api::v5::MangaDexClient;
    ///
    /// # async fn run() -> anyhow::Result<()> {
    /// let client = MangaDexClient::default();
    ///
    /// let res = client
    ///     .settings()
    ///     .create_or_update_user_settings()
    ///     .build()?
    ///     .send()
    ///     .await?;
    ///
    /// println!("results: {:?}", res);
    /// # Ok(())
    /// # }
    /// ```
    #[allow(unused)]
    fn create_or_update_user_settings(&self) -> CreateOrUpdateUserSettingsBuilder {
        CreateOrUpdateUserSettingsBuilder::default().http_client(self.http_client.clone())
    }

    /// Create a Settings template.
    ///
    /// <https://api.mangadex.org/swagger.html#/Settings/post-settings-template>
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use mangadex_api::v5::MangaDexClient;
    ///
    /// # async fn run() -> anyhow::Result<()> {
    /// let client = MangaDexClient::default();
    ///
    /// let res = client
    ///     .settings()
    ///     .create_template()
    ///     .build()?
    ///     .send()
    ///     .await?;
    ///
    /// println!("results: {:?}", res);
    /// # Ok(())
    /// # }
    /// ```
    #[allow(unused)]
    fn create_template(&self) -> CreateSettingsTemplateBuilder {
        CreateSettingsTemplateBuilder::default().http_client(self.http_client.clone())
    }

    /// Get the latest Settings template.
    ///
    /// <https://api.mangadex.org/swagger.html#/Settings/get-settings-template>
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use mangadex_api::v5::MangaDexClient;
    /// use uuid::Uuid;
    ///
    /// # async fn run() -> anyhow::Result<()> {
    /// let client = MangaDexClient::default();
    ///
    /// let res = client
    ///     .settings()
    ///     .get_latest_template()
    ///     .build()?
    ///     .send()
    ///     .await?;
    ///
    /// println!("res: {:?}", res);
    /// # Ok(())
    /// # }
    /// ```
    #[allow(unused)]
    fn get_latest_template(&self) -> GetLatestSettingsTemplateBuilder {
        GetLatestSettingsTemplateBuilder::default().http_client(self.http_client.clone())
    }

    /// Get a Settings template by version ID.
    ///
    /// <https://api.mangadex.org/swagger.html#/Settings/get-settings-template-version>
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use mangadex_api::v5::MangaDexClient;
    /// use uuid::Uuid;
    ///
    /// # async fn run() -> anyhow::Result<()> {
    /// let client = MangaDexClient::default();
    ///
    /// let version_id = Uuid::new_v4();
    /// let res = client
    ///     .settings()
    ///     .get_template_by_version_id()
    ///     .version(&version_id)
    ///     .build()?
    ///     .send()
    ///     .await?;
    ///
    /// println!("res: {:?}", res);
    /// # Ok(())
    /// # }
    /// ```
    #[allow(unused)]
    fn get_template_by_version_id(&self) -> GetSettingsTemplateByVersionIdBuilder {
        GetSettingsTemplateByVersionIdBuilder::default().http_client(self.http_client.clone())
    }

    /// Get a user's Settings.
    ///
    /// <https://api.mangadex.org/swagger.html#/Settings/get-settings>
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use mangadex_api::v5::MangaDexClient;
    /// use uuid::Uuid;
    ///
    /// # async fn run() -> anyhow::Result<()> {
    /// let client = MangaDexClient::default();
    ///
    /// let version_id = Uuid::new_v4();
    /// let res = client
    ///     .settings()
    ///     .get_user_settings()
    ///     .build()?
    ///     .send()
    ///     .await?;
    ///
    /// println!("res: {:?}", res);
    /// # Ok(())
    /// # }
    /// ```
    #[allow(unused)]
    fn get_user_settings(&self) -> GetUserSettingsBuilder {
        GetUserSettingsBuilder::default().http_client(self.http_client.clone())
    }
}
