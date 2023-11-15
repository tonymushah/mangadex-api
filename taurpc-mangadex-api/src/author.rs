pub mod create;
pub mod edit;
pub mod get_unique;
pub mod list;

use create::AuthorCreateParams;
use edit::AuthorEditParams;
use get_unique::AuthorGetUniqueParam;
use list::AuthorListParams;

use mangadex_api_schema::{
    v5::{AuthorCollection, AuthorData},
    Limited, NoData,
};
use tauri::{Runtime, Window};
use uuid::Uuid;

#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{
    v5::author::{
        get::ListAuthorBuilder,
        id::{get::GetAuthorBuilder, put::UpdateAuthorBuilder},
        post::CreateAuthorBuilder,
    },
    MangaDexClient,
};

use crate::Result;

#[taurpc::procedures(path = "mangadex_author", export_to = "../src/lib/mangadex_7.ts")]
pub trait Author {
    #[taurpc(alias = "list")]
    async fn list<R: Runtime>(
        params: AuthorListParams,
        window: Window<R>,
    ) -> Result<AuthorCollection>;
    async fn create<R: Runtime>(
        params: AuthorCreateParams,
        window: Window<R>,
    ) -> Result<Limited<AuthorData>>;
    async fn get_unique<R: Runtime>(
        params: AuthorGetUniqueParam,
        window: Window<R>,
    ) -> Result<AuthorData>;
    async fn edit<R: Runtime>(
        params: AuthorEditParams,
        window: Window<R>,
    ) -> Result<Limited<AuthorData>>;
    async fn delete<R: Runtime>(id: Uuid, window: Window<R>) -> Result<Limited<NoData>>;
}

#[cfg(feature = "mangadex-api-resolver")]
#[taurpc::resolvers]
impl Author for MangaDexClient {
    async fn list<R: Runtime>(
        self,
        params: AuthorListParams,
        _window: Window<R>,
    ) -> Result<AuthorCollection> {
        let builder: ListAuthorBuilder = params.into();
        builder
            .http_client(self.get_http_client())
            .send()
            .await
            .map_err(<crate::Error as From<mangadex_api_types::error::Error>>::from)
    }
    async fn create<R: Runtime>(
        self,
        params: AuthorCreateParams,
        _window: Window<R>,
    ) -> Result<Limited<AuthorData>> {
        let builder: CreateAuthorBuilder = params.into();
        builder
            .http_client(self.get_http_client())
            .send()
            .await
            .map_err(<crate::Error as From<mangadex_api_types::error::Error>>::from)
    }
    async fn get_unique<R: Runtime>(
        self,
        params: AuthorGetUniqueParam,
        _window: Window<R>,
    ) -> Result<AuthorData> {
        let builder: GetAuthorBuilder = params.into();
        builder
            .http_client(self.get_http_client())
            .send()
            .await
            .map_err(<crate::Error as From<mangadex_api_types::error::Error>>::from)
    }
    async fn edit<R: Runtime>(
        self,
        params: AuthorEditParams,
        _window: Window<R>,
    ) -> Result<Limited<AuthorData>> {
        let builder: UpdateAuthorBuilder = params.into();
        builder
            .http_client(self.get_http_client())
            .send()
            .await
            .map_err(<crate::Error as From<mangadex_api_types::error::Error>>::from)
    }
    async fn delete<R: Runtime>(self, id: Uuid, _window: Window<R>) -> Result<Limited<NoData>> {
        self.author()
            .id(id)
            .delete()
            .send()
            .await
            .map_err(<crate::Error as From<mangadex_api_types::error::Error>>::from)
    }
}

#[cfg(feature = "mangadex-api-resolver")]
#[derive(Debug, Clone)]
pub struct AuthorResolver(pub MangaDexClient);

#[cfg(feature = "mangadex-api-resolver")]
#[taurpc::resolvers]
impl Author for AuthorResolver {
    async fn list<R: Runtime>(
        self,
        params: AuthorListParams,
        window: Window<R>,
    ) -> Result<AuthorCollection> {
        self.0.list(params, window).await
    }
    async fn create<R: Runtime>(
        self,
        params: AuthorCreateParams,
        _window: Window<R>,
    ) -> Result<Limited<AuthorData>> {
        self.0.create(params, _window).await
    }
    async fn get_unique<R: Runtime>(
        self,
        params: AuthorGetUniqueParam,
        _window: Window<R>,
    ) -> Result<AuthorData> {
        self.0.get_unique(params, _window).await
    }
    async fn edit<R: Runtime>(
        self,
        params: AuthorEditParams,
        _window: Window<R>,
    ) -> Result<Limited<AuthorData>> {
        self.0.edit(params, _window).await
    }
    async fn delete<R: Runtime>(self, id: Uuid, _window: Window<R>) -> Result<Limited<NoData>> {
        self.0.delete(id, _window).await
    }
}
