use super::{UploadSessionFileAttributes, ApiObject, UploadSessionFileData, Result};

pub type UploadSessionFileObject = ApiObject<UploadSessionFileAttributes>;
pub type UploadSessionFileDataObject = UploadSessionFileData<UploadSessionFileObject>;
pub type UploadSessionFileResponse = Result<UploadSessionFileDataObject>;