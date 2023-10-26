use super::{ForumThreadAttributes, ForumThreadObject, Result};
use crate::ApiData;

pub type ForumThread = ForumThreadObject<ForumThreadAttributes>;
pub type ForumThreadResponseData = ApiData<ForumThread>;
pub type ForumThreadResponse = Result<ForumThreadResponseData>;
