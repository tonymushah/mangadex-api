use super::{ApiObjectNoRelationships, ReportReasonAttributes, Result, Results};

pub type ReportReasonObject = ApiObjectNoRelationships<ReportReasonAttributes>;
pub type ReportReasonCollection = Results<ReportReasonObject>;
pub type ReportReasonListResponse = Result<ReportReasonCollection>;
