pub mod create;
pub mod delete;
pub mod edit;
pub mod get_unique;
pub mod list;

use crate::Result;
use list::ApiClientListParam;

#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::MangaDexClient;

use mangadex_api_schema::{
    v5::{ApiClientCollection, ApiClientData, ApiClientSecret},
    NoData,
};
use tauri::{Runtime, Window};

use self::{
    create::ApiClientCreateParams, delete::ApiClientDeleteParam, edit::ApiClientEditParam,
    get_unique::ApiClientGetUniqueParams,
};

#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::v5::api_client::{
    get::ListClientsBuilder,
    id::{delete::DeleteClientBuilder, get::GetClientBuilder, post::EditClientBuilder},
    post::CreateClientBuilder,
};

use uuid::Uuid;

#[taurpc::procedures(path = "mangadex_api_client")]
pub trait ApiClient {
    #[taurpc(alias = "list")]
    async fn list<R: Runtime>(
        params: ApiClientListParam,
        window: Window<R>,
    ) -> Result<ApiClientCollection>;
    async fn create<R: Runtime>(
        params: ApiClientCreateParams,
        window: Window<R>,
    ) -> Result<ApiClientData>;
    async fn get_unique<R: Runtime>(
        params: ApiClientGetUniqueParams,
        window: Window<R>,
    ) -> Result<ApiClientData>;
    async fn edit<R: Runtime>(
        params: ApiClientEditParam,
        window: Window<R>,
    ) -> Result<ApiClientData>;
    async fn delete<R: Runtime>(params: ApiClientDeleteParam, window: Window<R>) -> Result<NoData>;
    async fn get_secret<R: Runtime>(id: Uuid, window: Window<R>) -> Result<ApiClientSecret>;
    async fn refresh_secret<R: Runtime>(id: Uuid, window: Window<R>) -> Result<ApiClientSecret>;
}

#[cfg(feature = "mangadex-api-resolver")]
#[taurpc::resolvers]
impl ApiClient for MangaDexClient {
    async fn list<R: Runtime>(
        self,
        params: ApiClientListParam,
        _window: Window<R>,
    ) -> Result<ApiClientCollection> {
        let builder = <ListClientsBuilder as From<ApiClientListParam>>::from(params);
        builder
            .http_client(self.get_http_client().clone())
            .send()
            .await
            .map_err(<crate::Error as From<mangadex_api_types::error::Error>>::from)
    }
    async fn create<R: Runtime>(
        self,
        params: ApiClientCreateParams,
        _window: Window<R>,
    ) -> Result<ApiClientData> {
        let builder = <CreateClientBuilder as From<ApiClientCreateParams>>::from(params);
        builder
            .http_client(self.get_http_client().clone())
            .send()
            .await
            .map_err(<crate::Error as From<mangadex_api_types::error::Error>>::from)
    }
    async fn get_unique<R: Runtime>(
        self,
        params: ApiClientGetUniqueParams,
        _window: Window<R>,
    ) -> Result<ApiClientData> {
        let builder: GetClientBuilder = params.into();
        builder
            .http_client(self.get_http_client().clone())
            .send()
            .await
            .map_err(<crate::Error as From<mangadex_api_types::error::Error>>::from)
    }
    async fn edit<R: Runtime>(
        self,
        params: ApiClientEditParam,
        _window: Window<R>,
    ) -> Result<ApiClientData> {
        let builder: EditClientBuilder = params.into();
        builder
            .http_client(self.get_http_client().clone())
            .send()
            .await
            .map_err(<crate::Error as From<mangadex_api_types::error::Error>>::from)
    }
    async fn delete<R: Runtime>(
        self,
        params: ApiClientDeleteParam,
        _window: Window<R>,
    ) -> Result<NoData> {
        let builder: DeleteClientBuilder = params.into();
        builder
            .http_client(self.get_http_client().clone())
            .send()
            .await
            .map_err(<crate::Error as From<mangadex_api_types::error::Error>>::from)?;
        Ok(NoData::default())
    }
    async fn get_secret<R: Runtime>(self, id: Uuid, _window: Window<R>) -> Result<ApiClientSecret> {
        self.client()
            .id(id)
            .secret()
            .get()
            .send()
            .await
            .map_err(<crate::Error as From<mangadex_api_types::error::Error>>::from)
    }
    async fn refresh_secret<R: Runtime>(
        self,
        id: Uuid,
        _window: Window<R>,
    ) -> Result<ApiClientSecret> {
        self.client()
            .id(id)
            .secret()
            .post()
            .send()
            .await
            .map_err(<crate::Error as From<mangadex_api_types::error::Error>>::from)
    }
}

#[cfg(feature = "mangadex-api-resolver")]
#[derive(Clone)]
pub struct ApiClientResolver(pub MangaDexClient);

#[cfg(feature = "mangadex-api-resolver")]
impl From<ApiClientResolver> for MangaDexClient {
    fn from(value: ApiClientResolver) -> Self {
        value.0
    }
}

#[cfg(feature = "mangadex-api-resolver")]
#[taurpc::resolvers]
impl ApiClient for ApiClientResolver {
    async fn list<R: Runtime>(
        self,
        params: ApiClientListParam,
        window: Window<R>,
    ) -> Result<ApiClientCollection> {
        let client: MangaDexClient = self.into();
        client.list(params, window).await
    }
    async fn create<R: Runtime>(
        self,
        params: ApiClientCreateParams,
        window: Window<R>,
    ) -> Result<ApiClientData> {
        let client: MangaDexClient = self.into();
        client.create(params, window).await
    }
    async fn get_unique<R: Runtime>(
        self,
        params: ApiClientGetUniqueParams,
        window: Window<R>,
    ) -> Result<ApiClientData> {
        let client: MangaDexClient = self.into();
        client.get_unique(params, window).await
    }
    async fn edit<R: Runtime>(
        self,
        params: ApiClientEditParam,
        window: Window<R>,
    ) -> Result<ApiClientData> {
        let client: MangaDexClient = self.into();
        client.edit(params, window).await
    }
    async fn delete<R: Runtime>(
        self,
        params: ApiClientDeleteParam,
        window: Window<R>,
    ) -> Result<NoData> {
        let client: MangaDexClient = self.into();
        client.delete(params, window).await
    }
    async fn get_secret<R: Runtime>(self, id: Uuid, window: Window<R>) -> Result<ApiClientSecret> {
        let client: MangaDexClient = self.into();
        client.get_secret(id, window).await
    }
    async fn refresh_secret<R: Runtime>(
        self,
        id: Uuid,
        window: Window<R>,
    ) -> Result<ApiClientSecret> {
        let client: MangaDexClient = self.into();
        client.refresh_secret(id, window).await
    }
}
