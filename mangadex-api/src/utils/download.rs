pub mod chapter;
pub mod cover;

use bytes::Bytes;
use uuid::Uuid;

use crate::HttpClientRef;

use self::{
    chapter::ChapterDownloadBuilder,
    cover::{CoverDownloadBuilder, CoverQuality},
};

use mangadex_api_types::error::Result;

pub type DownloadElement = (String, Result<Bytes>);

#[derive(Debug)]
pub struct DownloadBuilder {
    http_client: HttpClientRef,
}

impl DownloadBuilder {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }
    pub fn chapter(&self, id: Uuid) -> ChapterDownloadBuilder {
        ChapterDownloadBuilder::default()
            .http_client(self.http_client.clone())
            .id(id)
            .force_port_443(false)
            .report(false)
    }

    pub fn cover(&self) -> CoverDownloadBuilder {
        CoverDownloadBuilder::default()
            .http_client(self.http_client.clone())
            .quality(CoverQuality::Default)
    }
}
