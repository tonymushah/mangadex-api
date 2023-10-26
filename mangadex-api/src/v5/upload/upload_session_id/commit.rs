pub mod post;

use crate::HttpClientRef;

use uuid::Uuid;

use post::CommitUploadSessionBuilder;

/// Statistics endpoint handler builder.
#[derive(Clone, Debug)]
pub struct CommitEndpoint {
    http_client: HttpClientRef,
    upload_session_id: Uuid,
}

impl CommitEndpoint {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef, upload_session_id: Uuid) -> Self {
        Self {
            http_client,
            upload_session_id,
        }
    }

    pub fn post(&self) -> CommitUploadSessionBuilder {
        CommitUploadSessionBuilder::new(self.http_client.clone()).session_id(self.upload_session_id)
    }
}
