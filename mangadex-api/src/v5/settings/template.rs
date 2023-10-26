pub mod get;
pub mod post;
pub mod version;

use get::GetLatestSettingsTemplateBuilder;
use post::CreateSettingsTemplateBuilder;
use uuid::Uuid;
use version::SettingsTemplateVersionBuilder;

use crate::HttpClientRef;

#[derive(Debug)]
pub struct SettingsTemplateBuilder {
    #[allow(unused)]
    http_client: HttpClientRef,
}

impl SettingsTemplateBuilder {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }
    pub fn get(&self) -> GetLatestSettingsTemplateBuilder {
        GetLatestSettingsTemplateBuilder::default().http_client(self.http_client.clone())
    }
    pub fn post(&self) -> CreateSettingsTemplateBuilder {
        CreateSettingsTemplateBuilder::default().http_client(self.http_client.clone())
    }
    pub fn version(&self, id: Uuid) -> SettingsTemplateVersionBuilder {
        SettingsTemplateVersionBuilder::new(self.http_client.clone(), id)
    }
}
