use super::{ApiClientAttributes, ApiData, ApiObject, Result, Results};

pub type ApiClientObject = ApiObject<ApiClientAttributes>;
pub type ApiClientData = ApiData<ApiClientObject>;
pub type ApiClientResponse = Result<ApiClientData>;
pub type ApiClientCollection = Results<ApiClientObject>;
pub type ApiClientListResponse = Result<ApiClientCollection>;
