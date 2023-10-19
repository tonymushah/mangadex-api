pub mod get;
use uuid::Uuid;

use get::GetSettingsTemplateByVersionIdBuilder;

use crate::HttpClientRef;

#[derive(Debug)]
pub struct SettingsTemplateVersionBuilder {
    #[allow(unused)]
    http_client: HttpClientRef,
    version: Uuid,
}

impl SettingsTemplateVersionBuilder {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef, version: Uuid) -> Self {
        Self {
            http_client,
            version,
        }
    }
    pub fn get(&self) -> GetSettingsTemplateByVersionIdBuilder {
        GetSettingsTemplateByVersionIdBuilder::default()
            .version(self.version)
            .http_client(self.http_client.clone())
    }
}
