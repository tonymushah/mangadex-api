use reqwest::Client;

use crate::MangaDexClient;

pub mod download;

/// Gives you the `reqwest::Client` from the `MangaDexClient`
/// Comes handy when you don't want to build a new `reqwest` Client
pub async fn get_reqwest_client(client: &MangaDexClient) -> Client {
    #[cfg(not(feature = "multi-thread"))]
    {
        client.get_http_client().borrow().client.clone()
    }
    #[cfg(feature = "multi-thread")]
    {
        client.get_http_client().lock().await.client.clone()
    }
}
