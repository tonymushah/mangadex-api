use crate::ApiData;
use super::{ForumThreadObject, ForumThreadAttributes, Result};

pub type ForumThread = ForumThreadObject<ForumThreadAttributes>;
pub type ForumThreadResponseData = ApiData<ForumThread>;
pub type ForumThreadResponse = Result<ForumThreadResponseData>;