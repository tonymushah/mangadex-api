use super::{ApiData, ApiObject, CustomListAttributes, Results};

pub type CustomListObject = ApiObject<CustomListAttributes>;
pub type CustomListData = ApiData<CustomListObject>;
pub type CustomListCollection = Results<CustomListObject>;
