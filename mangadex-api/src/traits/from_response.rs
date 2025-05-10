use mangadex_api_schema::{
    v5::error::MangaDexErrorResponse, v5::Results, ApiData, ApiObject, ApiObjectNoRelationships,
    ApiResult, NoData,
};
use serde::de::DeserializeOwned;

use super::FromResponse;

impl FromResponse for NoData {
    type Response = Self;
    fn from_response(res: Self::Response) -> Self {
        res
    }
}

impl<T> FromResponse for Result<T, crate::error::Error> {
    type Response = ApiResult<T, MangaDexErrorResponse>;

    fn from_response(value: Self::Response) -> Self {
        value.into_result().map_err(|e| e.into())
    }
}

impl<T> FromResponse for Vec<Result<T, crate::error::Error>> {
    type Response = Vec<ApiResult<T, MangaDexErrorResponse>>;

    fn from_response(value: Self::Response) -> Self {
        value
            .into_iter()
            .map(|r| r.into_result().map_err(|e| e.into()))
            .collect()
    }
}

impl<A> FromResponse for ApiObjectNoRelationships<A> {
    type Response = Self;

    fn from_response(value: Self::Response) -> Self {
        value
    }
}

impl<A> FromResponse for ApiObject<A> {
    type Response = Self;

    fn from_response(value: Self::Response) -> Self {
        value
    }
}

impl<T> FromResponse for ApiData<T>
where
    T: DeserializeOwned,
{
    type Response = Self;

    fn from_response(value: Self::Response) -> Self {
        value
    }
}

impl<T> FromResponse for Results<T> {
    type Response = Self;
    fn from_response(res: Self::Response) -> Self {
        res
    }
}

impl FromResponse for mangadex_api_schema::v5::AtHomeServer {
    type Response = Self;
    fn from_response(res: Self::Response) -> Self {
        res
    }
}

impl FromResponse for mangadex_api_schema::v5::CheckUsernameAvailableResponse {
    type Response = Self;

    fn from_response(value: Self::Response) -> Self {
        value
    }
}

impl FromResponse for mangadex_api_schema::v5::oauth::OAuthTokenResponse {
    type Response = Self;

    fn from_response(res: Self::Response) -> Self {
        res
    }
}

impl FromResponse for mangadex_api_schema::v5::RefreshTokenResponse {
    type Response = Self;

    fn from_response(res: Self::Response) -> Self {
        res
    }
}

impl FromResponse for mangadex_api_schema::v5::upload_required_approval::UploadRequiredApproval {
    type Response = Self;
    fn from_response(res: Self::Response) -> Self {
        res
    }
}

impl FromResponse for mangadex_api_schema::v5::MangaStatisticsObject {
    type Response = Self;

    fn from_response(value: Self::Response) -> Self {
        value
    }
}

impl<A, T> FromResponse for mangadex_api_schema::v5::ForumThreadObject<A, T> {
    type Response = Self;

    fn from_response(value: Self::Response) -> Self {
        value
    }
}

impl<A> FromResponse for mangadex_api_schema::v5::UploadSessionFileData<A> {
    type Response = Self;
    fn from_response(res: Self::Response) -> Self {
        res
    }
}
