use super::{ApiData, ApiObject, CoverAttributes, Result, Results};

pub type CoverObject = ApiObject<CoverAttributes>;
pub type CoverData = ApiData<CoverObject>;
pub type CoverResponse = Result<CoverData>;
pub type CoverCollection = Results<CoverObject>;
pub type CoverListResponse = Result<CoverCollection>;
