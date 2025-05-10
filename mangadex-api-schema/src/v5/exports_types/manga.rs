use super::{ApiData, ApiObject, MangaAttributes, MangaRelationAttributes, Results};

pub type MangaObject = ApiObject<MangaAttributes>;
pub type MangaData = ApiData<MangaObject>;
pub type MangaCollection = Results<MangaObject>;

pub type MangaRelationObject = ApiObject<MangaRelationAttributes>;
pub type MangaRelationCollection = Results<MangaRelationObject>;
