use super::{ApiData, ApiObject, AuthorAttributes, Results};

pub type AuthorObject = ApiObject<AuthorAttributes>;
pub type AuthorData = ApiData<AuthorObject>;
pub type AuthorCollection = Results<AuthorObject>;
