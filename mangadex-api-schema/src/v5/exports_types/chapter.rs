use super::{ApiData, ApiObject, ChapterAttributes, Results};

pub type ChapterObject = ApiObject<ChapterAttributes>;
pub type ChapterData = ApiData<ChapterObject>;
pub type ChapterCollection = Results<ChapterObject>;
