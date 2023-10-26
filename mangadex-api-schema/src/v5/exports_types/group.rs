use super::{
    ApiData, ApiObject, GroupStatisticsObject, Result, Results, ScanlationGroupAttributes,
};

pub type GroupObject = ApiObject<ScanlationGroupAttributes>;
pub type GroupData = ApiData<GroupObject>;
pub type GroupResponse = Result<GroupData>;
pub type GroupCollection = Results<GroupObject>;
pub type GroupListResponse = Result<GroupCollection>;

pub type GroupStatisticsResponse = Result<GroupStatisticsObject>;
