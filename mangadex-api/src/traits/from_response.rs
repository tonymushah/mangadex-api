impl FromResponse for NoData {
    type Response = Self;
    fn from_response(res: Self::Response) -> Self {
        res
    }
}

impl<T> FromResponse for Result<T, Error> {
    type Response = ApiResult<T, MangaDexErrorResponse>;

    fn from_response(value: Self::Response) -> Self {
        value.into_result().map_err(|e| e.into())
    }
}

impl<T> FromResponse for Vec<Result<T, Error>> {
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

impl FromResponse for AtHomeServer {
    type Response = Self;
    fn from_response(res: Self::Response) -> Self {
        res
    }
}

impl FromResponse for CheckUsernameAvailableResponse {
    type Response = Self;

    fn from_response(value: Self::Response) -> Self {
        value
    }
}

impl FromResponse for CheckUsernameAvailableResponse {
    type Response = Self;

    fn from_response(value: Self::Response) -> Self {
        value
    }
}

impl FromResponse for OAuthTokenResponse {
    type Response = Self;

    fn from_response(res: Self::Response) -> Self {
        res
    }
}

impl FromResponse for RefreshTokenResponse {
    type Response = Self;

    fn from_response(res: Self::Response) -> Self {
        res
    }
}

impl FromResponse for UploadRequiredApproval {
    type Response = Self;
    fn from_response(res: Self::Response) -> Self {
        res
    }
}

impl FromResponse for MangaStatisticsObject {
    type Response = Self;

    fn from_response(value: Self::Response) -> Self {
        value
    }
}

impl<A, T> FromResponse for ForumThreadObject<A, T> {
    type Response = Self;

    fn from_response(value: Self::Response) -> Self {
        value
    }
}

impl<A> FromResponse for UploadSessionFileData<A> {
    type Response = Self;
    fn from_response(res: Self::Response) -> Self {
        res
    }
}
