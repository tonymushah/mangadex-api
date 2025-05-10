use super::{ApiObject, UploadSessionFileAttributes, UploadSessionFileData};

use crate::{ApiData, v5::upload_session::UploadSessionAttributes};

pub type UploadSessionObject = ApiObject<UploadSessionAttributes>;
pub type UploadSessionData = ApiData<UploadSessionObject>;

pub type UploadSessionFileObject = ApiObject<UploadSessionFileAttributes>;
pub type UploadSessionFileDataObject = UploadSessionFileData<UploadSessionFileObject>;
