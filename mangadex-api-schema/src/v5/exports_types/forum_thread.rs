use super::{ForumThreadAttributes, ForumThreadObject};
use crate::ApiData;

pub type ForumThread = ForumThreadObject<ForumThreadAttributes>;
pub type ForumThreadResponseData = ApiData<ForumThread>;
