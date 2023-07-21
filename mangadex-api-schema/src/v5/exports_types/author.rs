use super::{ApiData, ApiObject, AuthorAttributes, Result, Results};

pub type AuthorObject = ApiObject<AuthorAttributes>;
pub type AuthorData = ApiData<AuthorObject>;
pub type AuthorResponse = Result<AuthorData>;
pub type AuthorCollection = Results<AuthorObject>;
pub type AuthorListResponse = Result<AuthorCollection>;
