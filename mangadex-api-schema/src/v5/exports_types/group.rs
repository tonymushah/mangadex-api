use super::{ApiData, ApiObject, ScanlationGroupAttributes, Result, Results, GroupStatisticsObject};

pub type GroupObject = ApiObject<ScanlationGroupAttributes>;
pub type GroupData = ApiData<GroupObject>;
pub type GroupResponse = Result<GroupData>;
pub type GroupCollection = Results<GroupObject>;
pub type GroupListResponse = Result<GroupCollection>;

pub type GroupStatisticsResponse = Result<GroupStatisticsObject>;