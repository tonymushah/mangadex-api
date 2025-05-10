use super::{ApiData, ApiObject, CoverAttributes, Results};

pub type CoverObject = ApiObject<CoverAttributes>;
pub type CoverData = ApiData<CoverObject>;
pub type CoverCollection = Results<CoverObject>;
