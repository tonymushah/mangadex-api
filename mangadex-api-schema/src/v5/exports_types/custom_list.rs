use super::{ApiData, ApiObject, CustomListAttributes, Result, Results};

pub type CustomListObject = ApiObject<CustomListAttributes>;
pub type CustomListData = ApiData<CustomListObject>;
pub type CustomListResponse = Result<CustomListData>;
pub type CustomListCollection = Results<CustomListObject>;
pub type CustomListListResponse = Result<CustomListCollection>;
