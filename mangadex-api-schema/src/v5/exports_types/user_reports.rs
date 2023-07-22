use super::{UserReportAttributes, ApiObject, ApiData, Results, Result};

pub type UserReportsObject = ApiObject<UserReportAttributes>;
pub type UserReportsData = ApiData<UserReportsObject>;
pub type UserReportsCollection = Results<UserReportsObject>;
pub type UserReportsListResponse = Result<UserReportsCollection>;