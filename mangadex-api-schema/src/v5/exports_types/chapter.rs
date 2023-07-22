use super::{ApiData, ApiObject, ChapterAttributes, Result, Results, ChapterStatisticsObject};

pub type ChapterObject = ApiObject<ChapterAttributes>;
pub type ChapterData = ApiData<ChapterObject>;
pub type ChapterResponse = Result<ChapterData>;
pub type ChapterCollection = Results<ChapterObject>;
pub type ChapterListResponse = Result<ChapterCollection>;

pub type ChapterStatisticsResponse = Result<ChapterStatisticsObject>;