pub mod post;
use crate::HttpClientRef;
use post::SolveCaptchaBuilder;

#[derive(Clone, Debug)]
pub struct SolveEndpoint {
    http_client: HttpClientRef,
}

impl SolveEndpoint {
    #[doc(hidden)]
    pub fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }
    pub fn post(&self) -> SolveCaptchaBuilder {
        SolveCaptchaBuilder::default().http_client(self.http_client.clone())
    }
}
