pub mod batch_manga;
#[cfg(feature = "custom_list_v2")]
pub mod bookmark;
#[cfg(feature = "custom_list_v2")]
pub mod default;
pub mod delete;
pub mod feed;
pub mod follow;
pub mod get;
#[cfg(feature = "custom_list_v2")]
pub mod manga;
#[cfg(feature = "custom_list_v2")]
pub mod pin;
pub mod put;
#[cfg(feature = "custom_list_v2")]
pub mod unpin;

use batch_manga::BatchMangaEndpoint;
#[cfg(feature = "custom_list_v2")]
use bookmark::BookMarkEndpoint;
#[cfg(feature = "custom_list_v2")]
use default::DefaultEndpoint;
use delete::DeleteCustomListBuilder;
use feed::FeedEndPoint;
use follow::FollowEndpoint;
use get::GetCustomListBuilder;
#[cfg(feature = "custom_list_v2")]
use manga::MangaEndpoint;
#[cfg(feature = "custom_list_v2")]
use pin::PinEndpoint;
use put::UpdateCustomListBuilder;
#[cfg(feature = "custom_list_v2")]
use unpin::UnPinEndpoint;

use uuid::Uuid;

use crate::HttpClientRef;

pub struct IdEnpoint {
    http_client: HttpClientRef,
    id: Uuid,
}

impl IdEnpoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef, id: Uuid) -> Self {
        Self { http_client, id }
    }
    pub fn batch_manga(&self) -> BatchMangaEndpoint {
        BatchMangaEndpoint::new(self.http_client.clone(), self.id)
    }
    #[cfg(feature = "custom_list_v2")]
    pub fn bookmark(&self) -> BookMarkEndpoint {
        BookMarkEndpoint::new(self.http_client.clone(), self.id)
    }
    #[cfg(feature = "custom_list_v2")]
    pub fn default(&self) -> DefaultEndpoint {
        DefaultEndpoint::new(self.http_client.clone(), self.id)
    }
    pub fn feed(&self) -> FeedEndPoint {
        FeedEndPoint::new(self.http_client.clone(), self.id)
    }
    #[cfg(feature = "custom_list_v2")]
    pub fn manga(&self) -> MangaEndpoint {
        MangaEndpoint::new(self.http_client.clone(), self.id)
    }
    pub fn get(&self) -> GetCustomListBuilder {
        GetCustomListBuilder::default()
            .list_id(self.id)
            .http_client(self.http_client.clone())
    }
    #[cfg(feature = "custom_list_v2")]
    pub fn pin(&self) -> PinEndpoint {
        PinEndpoint::new(self.http_client.clone(), self.id)
    }
    #[cfg(feature = "custom_list_v2")]
    pub fn unpin(&self) -> UnPinEndpoint {
        UnPinEndpoint::new(self.http_client.clone(), self.id)
    }
    pub fn delete(&self) -> DeleteCustomListBuilder {
        DeleteCustomListBuilder::default()
            .list_id(self.id)
            .http_client(self.http_client.clone())
    }
    pub fn put(&self) -> UpdateCustomListBuilder {
        UpdateCustomListBuilder::default()
            .list_id(self.id)
            .http_client(self.http_client.clone())
    }
    /// Pretty much the same as `.bookmark()`
    #[cfg_attr(
        feature = "custom_list_v2",
        deprecated(since = "3.0.0-alpha.1", note = "Use `.bookmark()` instead")
    )]
    pub fn follow(&self) -> FollowEndpoint {
        FollowEndpoint::new(self.http_client.to_owned(), self.id)
    }
}
