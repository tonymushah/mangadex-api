use super::{ApiObjectNoRelationships, ReportReasonAttributes, Results, Result};

pub type ReportReasonObject = ApiObjectNoRelationships<ReportReasonAttributes>;
pub type ReportReasonCollection = Results<ReportReasonObject>;
pub type ReportReasonListResponse = Result<ReportReasonCollection>;