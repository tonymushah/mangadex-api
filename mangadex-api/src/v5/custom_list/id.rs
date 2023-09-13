pub mod batch_manga;
pub mod bookmark;
pub mod default;
pub mod feed;
pub mod manga;
pub mod get;
pub mod pin;
pub mod unpin;
pub mod delete;
pub mod put;

use batch_manga::BatchMangaEndpoint;
use bookmark::BookMarkEndpoint;
use default::DefaultEndpoint;
use feed::FeedEndPoint;
use manga::MangaEndpoint;
use get::GetCustomListBuilder;
use pin::PinEndpoint;
use unpin::UnPinEndpoint;
use delete::DeleteCustomListBuilder;
use put::UpdateCustomListBuilder;

use uuid::Uuid;

use crate::HttpClientRef;

pub struct IdEnpoint{
    http_client: HttpClientRef,
    id: Uuid
}

impl IdEnpoint{
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef, id: Uuid) -> Self{
        Self{
            http_client,
            id
        }
    }
    pub fn batch_manga(&self) -> BatchMangaEndpoint{
        BatchMangaEndpoint::new(self.http_client.clone(), self.id)
    }
    pub fn bookmark(&self) -> BookMarkEndpoint{
        BookMarkEndpoint::new(self.http_client.clone(), self.id)
    }
    pub fn default(&self) -> DefaultEndpoint{
        DefaultEndpoint::new(self.http_client.clone(), self.id)
    }
    pub fn feed(&self) -> FeedEndPoint{
        FeedEndPoint::new(self.http_client.clone(), self.id)
    }
    pub fn manga(&self) -> MangaEndpoint {
        MangaEndpoint::new(self.http_client.clone(), self.id)
    }
    pub fn get(&self) -> GetCustomListBuilder{
        GetCustomListBuilder::default().http_client(self.http_client.clone()).list_id(self.id)
    }
    pub fn pin(&self) -> PinEndpoint{
        PinEndpoint::new(self.http_client.clone(), self.id)
    }
    pub fn unpin(&self) -> UnPinEndpoint{
        UnPinEndpoint::new(self.http_client.clone(), self.id)
    }
    pub fn delete(&self) -> DeleteCustomListBuilder{
        DeleteCustomListBuilder::default().http_client(self.http_client.clone()).list_id(self.id)
    }
    pub fn put(&self) -> UpdateCustomListBuilder{
        UpdateCustomListBuilder::default().http_client(self.http_client.clone()).list_id(self.id)
    }
    /// Same as `.bookmark()`
    #[deprecated(since = "3.0.0-rc1", note = "Use `.bookmark()` instead")]
    pub fn follow(&self) -> BookMarkEndpoint{
        self.bookmark()
    }
}