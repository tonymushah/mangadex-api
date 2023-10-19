//! Settings endpoint handler.
//!
//! <https://api.mangadex.org/swagger.html#/Settings>

pub mod get;
pub mod post;
pub mod template;

use crate::HttpClientRef;
use get::GetUserSettingsBuilder;
use post::CreateOrUpdateUserSettingsBuilder;
use template::SettingsTemplateBuilder;

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

    pub fn get(&self) -> GetUserSettingsBuilder {
        GetUserSettingsBuilder::default().http_client(self.http_client.clone())
    }
    pub fn post(&self) -> CreateOrUpdateUserSettingsBuilder {
        CreateOrUpdateUserSettingsBuilder::default().http_client(self.http_client.clone())
    }
    pub fn template(&self) -> SettingsTemplateBuilder {
        SettingsTemplateBuilder::new(self.http_client.clone())
    }
}
