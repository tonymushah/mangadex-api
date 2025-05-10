use super::{ApiObjectNoRelationships, ReportReasonAttributes, Results};

pub type ReportReasonObject = ApiObjectNoRelationships<ReportReasonAttributes>;
pub type ReportReasonCollection = Results<ReportReasonObject>;
