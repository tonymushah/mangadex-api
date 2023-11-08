pub mod delete;

use self::delete::DeleteImageBuilder;

use crate::HttpClientRef;

use uuid::Uuid;

/// Statistics endpoint handler builder.
#[derive(Clone, Debug)]
pub struct UploadSessionFileEndpoint {
    http_client: HttpClientRef,
    upload_session_id: Uuid,
    upload_session_file_id: Uuid,
}

impl UploadSessionFileEndpoint {
    #[doc(hidden)]
    pub(crate) fn new(
        http_client: HttpClientRef,
        upload_session_id: Uuid,
        upload_session_file_id: Uuid,
    ) -> Self {
        Self {
            http_client,
            upload_session_id,
            upload_session_file_id,
        }
    }
    pub fn delete(&self) -> DeleteImageBuilder {
        DeleteImageBuilder::default()
            .session_id(self.upload_session_id)
            .session_file_id(self.upload_session_file_id)
            .http_client(self.http_client.clone())
    }
}
