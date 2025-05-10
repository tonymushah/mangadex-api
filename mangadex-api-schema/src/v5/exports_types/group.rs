use super::{ApiData, ApiObject, Results, ScanlationGroupAttributes};

pub type GroupObject = ApiObject<ScanlationGroupAttributes>;
pub type GroupData = ApiData<GroupObject>;
pub type GroupCollection = Results<GroupObject>;
