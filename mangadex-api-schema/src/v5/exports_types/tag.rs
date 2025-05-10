use super::{ApiData, ApiObject, Results, TagAttributes};

pub type TagObject = ApiObject<TagAttributes>;
pub type TagData = ApiData<TagObject>;
pub type TagCollection = Results<TagObject>;
