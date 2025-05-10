pub mod from_response;

pub trait UrlSerdeQS {
    fn query_qs<T: Serialize>(self, query: &T) -> Self;
}

impl UrlSerdeQS for url::Url {
    fn query_qs<T: Serialize>(mut self, query: &T) -> Self {
        self.set_query(Some(
            &serde_qs::to_string(query).expect("failed to encode query string"),
        ));
        self
    }
}

pub trait FromResponse: Sized {
    type Response;

    fn from_response(res: Self::Response) -> Self;
}

pub trait Endpoint {
    type Query: Serialize;
    type Body: Serialize;
    type Response: FromResponse;

    fn method(&self) -> reqwest::Method {
        reqwest::Method::GET
    }

    fn path(&self) -> Cow<str>;

    fn require_auth(&self) -> bool {
        false
    }

    fn query(&self) -> Option<&Self::Query> {
        None
    }

    fn body(&self) -> Option<&Self::Body> {
        None
    }

    fn multipart(&self) -> Option<reqwest::multipart::Form> {
        None
    }
}
