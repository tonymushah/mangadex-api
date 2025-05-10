use super::{ApiData, ApiObject, LegacyMappingIdAttributes, Results};

pub type IdMappingObject = ApiObject<LegacyMappingIdAttributes>;
pub type IdMappingData = ApiData<IdMappingObject>;
pub type IdMappingCollection = Results<IdMappingObject>;
