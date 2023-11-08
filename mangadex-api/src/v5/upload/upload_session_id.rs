pub mod batch;
pub mod commit;
pub mod delete;
pub mod post;
pub mod upload_session_file_id;

use crate::HttpClientRef;

use uuid::Uuid;

use batch::BatchEndpoint;
use commit::CommitEndpoint;
use delete::AbandonUploadSessionBuilder;
use post::UploadImagesBuilder;
use upload_session_file_id::UploadSessionFileEndpoint;

/// Statistics endpoint handler builder.
#[derive(Clone, Debug)]
pub struct UploadSessionIdEndpoint {
    http_client: HttpClientRef,
    upload_session_id: Uuid,
}

impl UploadSessionIdEndpoint {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef, upload_session_id: Uuid) -> Self {
        Self {
            http_client,
            upload_session_id,
        }
    }
    pub fn batch(&self) -> BatchEndpoint {
        BatchEndpoint::new(self.http_client.clone(), self.upload_session_id)
    }
    pub fn commit(&self) -> CommitEndpoint {
        CommitEndpoint::new(self.http_client.clone(), self.upload_session_id)
    }
    pub fn delete(&self) -> AbandonUploadSessionBuilder {
        AbandonUploadSessionBuilder::default()
            .session_id(self.upload_session_id)
            .http_client(self.http_client.clone())
    }
    pub fn post(&self) -> UploadImagesBuilder {
        UploadImagesBuilder::default()
            .session_id(self.upload_session_id)
            .http_client(self.http_client.clone())
    }
    pub fn upload_session_file_id(
        &self,
        upload_session_file_id: Uuid,
    ) -> UploadSessionFileEndpoint {
        UploadSessionFileEndpoint::new(
            self.http_client.clone(),
            self.upload_session_id,
            upload_session_file_id,
        )
    }
}
