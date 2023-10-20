pub mod delete;

use self::delete::DeleteImagesBuilder;

use crate::HttpClientRef;

use uuid::Uuid;

/// Statistics endpoint handler builder.
#[derive(Clone, Debug)]
pub struct BatchEndpoint {
    http_client: HttpClientRef,
    upload_session_id: Uuid,
}

impl BatchEndpoint {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef, upload_session_id: Uuid) -> Self {
        Self {
            http_client,
            upload_session_id,
        }
    }
    pub fn delete(&self) -> DeleteImagesBuilder {
        DeleteImagesBuilder::default()
            .session_id(self.upload_session_id)
            .http_client(self.http_client.clone())
    }
}
