use super::{ApiObject, Result, UploadSessionFileAttributes, UploadSessionFileData};

pub type UploadSessionFileObject = ApiObject<UploadSessionFileAttributes>;
pub type UploadSessionFileDataObject = UploadSessionFileData<UploadSessionFileObject>;
pub type UploadSessionFileResponse = Result<UploadSessionFileDataObject>;
