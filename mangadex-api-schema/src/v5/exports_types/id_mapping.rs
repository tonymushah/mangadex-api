use super::{ApiData, ApiObject, LegacyMappingIdAttributes, Result, Results};

pub type IdMappingObject = ApiObject<LegacyMappingIdAttributes>;
pub type IdMappingData = ApiData<IdMappingObject>;
pub type IdMappindCollection = Results<IdMappingObject>;
pub type IdMappingListResponse = Result<IdMappindCollection>;
