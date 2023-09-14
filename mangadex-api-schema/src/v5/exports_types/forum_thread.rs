use crate::ApiData;
use crate::ApiResult;
use super::{ForumThreadObject, ForumThreadAttributes};

pub type ForumThread = ForumThreadObject<ForumThreadAttributes>;
pub type ForumThreadResponseData = ApiData<ForumThread>;
pub type ForumThreadResponse = ApiResult<ForumThreadResponseData>;