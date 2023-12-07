use reqwest::Client;

use crate::MangaDexClient;

pub mod download;
pub mod upload;

/// Gives you the `reqwest::Client` from the `MangaDexClient`
/// Comes handy when you don't want to build a new `reqwest` Client
pub async fn get_reqwest_client(client: &MangaDexClient) -> Client {
    #[cfg(all(
        not(feature = "multi-thread"),
        not(feature = "tokio-multi-thread"),
        not(feature = "rw-multi-thread")
    ))]
    {
        client.get_http_client().borrow().client.clone()
    }
    #[cfg(any(feature = "multi-thread", feature = "tokio-multi-thread"))]
    {
        client.get_http_client().lock().await.client.clone()
    }
    #[cfg(feature = "rw-multi-thread")]
    {
        client.get_http_client().read().await.client.clone()
    }
}
