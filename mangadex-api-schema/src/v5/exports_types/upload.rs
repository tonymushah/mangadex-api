use super::{ApiObject, Result, UploadSessionFileAttributes, UploadSessionFileData};

use crate::{v5::upload_session::UploadSessionAttributes, ApiData};

pub type UploadSessionObject = ApiObject<UploadSessionAttributes>;
pub type UploadSessionData = ApiData<UploadSessionObject>;
pub type UploadSessionResponse = Result<UploadSessionData>;

pub type UploadSessionFileObject = ApiObject<UploadSessionFileAttributes>;
pub type UploadSessionFileDataObject = UploadSessionFileData<UploadSessionFileObject>;
pub type UploadSessionFileResponse = Result<UploadSessionFileDataObject>;
