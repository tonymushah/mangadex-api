//! Upload endpoint handler.
//!
//! <https://api.mangadex.org/docs/swagger.html#/Upload>

use uuid::Uuid;

use crate::v5::cover::manga_id::post::UploadCoverBuilder;

use crate::HttpClientRef;

pub mod begin;
pub mod get;
pub mod upload_session_id;

use self::begin::BeginEndpoint;
use self::get::GetUploadSessionBuilder;
use self::upload_session_id::UploadSessionIdEndpoint;

/// Upload endpoint handler builder.
#[derive(Debug)]
pub struct UploadBuilder {
    http_client: HttpClientRef,
}

impl UploadBuilder {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }

    /// Upload a manga cover image.
    ///
    /// <https://api.mangadex.org/swagger.html#/Upload/upload-cover>
    pub fn cover(&self, manga_id: Uuid) -> UploadCoverBuilder {
        UploadCoverBuilder::default()
            .manga_id(manga_id)
            .http_client(self.http_client.clone())
    }

    pub fn get(&self) -> GetUploadSessionBuilder {
        GetUploadSessionBuilder::default().http_client(self.http_client.clone())
    }

    pub fn begin(&self) -> BeginEndpoint {
        BeginEndpoint::new(self.http_client.clone())
    }

    pub fn upload_session_id(&self, upload_session_id: Uuid) -> UploadSessionIdEndpoint {
        UploadSessionIdEndpoint::new(self.http_client.clone(), upload_session_id)
    }
}
