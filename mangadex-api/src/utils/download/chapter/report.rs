use std::{sync::Arc, time::Duration};

use mangadex_api_types::error::{Error, Result};
use reqwest::{Client, Response};
use serde::Serialize;
use url::Url;

/// Send a report to `https://api.mangadex.network/report`.
///
/// More details at : https://api.mangadex.org/docs/retrieving-chapter/#the-mangadexhome-report-endpoint
#[derive(Serialize, Clone)]
pub struct AtHomeReport {
    pub url: Url,
    pub success: bool,
    pub cached: bool,
    pub bytes: usize,
    pub duration: u128,
}

impl AtHomeReport {
    pub async fn send(&self, client: Arc<Client>) -> Result<Response> {
        if !self.url.as_str().contains("mangadex.org") {
            match client
                .post("https://api.mangadex.network/report")
                .json(self)
                .timeout(Duration::from_secs(1))
                .send()
                .await
            {
                Ok(d) => Result::Ok(d),
                Err(e) => Result::Err(Error::RequestError(e)),
            }
        } else {
            Result::Err(Error::unknow("the mangadex.org pattern found!"))
        }
    }
}
