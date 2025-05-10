use super::{ApiData, ApiObject, Results, UserReportAttributes};

pub type UserReportsObject = ApiObject<UserReportAttributes>;
pub type UserReportsData = ApiData<UserReportsObject>;
pub type UserReportsCollection = Results<UserReportsObject>;
