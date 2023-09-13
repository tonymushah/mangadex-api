use super::{
    ApiData, ApiObject, MangaAggregate, MangaAttributes, MangaReadMarkers, MangaReadingStatus,
    MangaReadingStatuses, MangaRelationAttributes, MangaStatisticsObject, Result, Results,
    UngroupedMangaReadMarkers,
};

pub type MangaObject = ApiObject<MangaAttributes>;
pub type MangaData = ApiData<MangaObject>;
pub type MangaResponse = Result<MangaData>;
pub type MangaCollection = Results<MangaObject>;
pub type MangaListResponse = Result<MangaCollection>;

pub type MangaAggregateResponse = Result<MangaAggregate>;

pub type UngroupedMangaReadMarkersResponse = Result<UngroupedMangaReadMarkers>;
pub type MangaReadMarkersResponse = Result<MangaReadMarkers>;

pub type MangaReadingStatusResponse = Result<MangaReadingStatus>;
pub type MangaReadingStatusesResponse = Result<MangaReadingStatuses>;

pub type MangaRelationObject = ApiObject<MangaRelationAttributes>;
pub type MangaRelationCollection = Results<MangaRelationObject>;
pub type MangaRelationListResponse = Result<MangaRelationCollection>;

pub type MangaStatisticsResponse = Result<MangaStatisticsObject>;
