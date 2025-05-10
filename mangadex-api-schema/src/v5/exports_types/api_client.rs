use super::{ApiClientAttributes, ApiData, ApiObject, Results};

pub type ApiClientObject = ApiObject<ApiClientAttributes>;
pub type ApiClientData = ApiData<ApiClientObject>;
pub type ApiClientCollection = Results<ApiClientObject>;
