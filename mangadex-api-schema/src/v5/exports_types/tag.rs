use super::{ApiData, ApiObject, Result, Results, TagAttributes};

pub type TagObject = ApiObject<TagAttributes>;
pub type TagData = ApiData<TagObject>;
pub type TagResponse = Result<TagData>;
pub type TagCollection = Results<TagObject>;
pub type TagListResponse = Result<TagCollection>;
