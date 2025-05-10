#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![cfg_attr(
    not(any(feature = "multi-thread", feature = "tokio-multi-thread")),
    allow(clippy::await_holding_refcell_ref)
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[macro_use]
pub(crate) mod macros;

pub mod constants;
#[macro_use]
pub mod http_client;
pub mod error;
pub mod rate_limit;
pub mod v5;

cfg_utils! {
    pub mod utils;
}

pub use constants::*;
pub use http_client::{HttpClient, HttpClientRef};
use reqwest::{
    header::{HeaderMap, HeaderValue, USER_AGENT},
    Client,
};
pub use v5::MangaDexClient;

pub type Result<T, E = error::Error> = std::result::Result<T, E>;

pub(crate) fn get_default_client_api() -> Client {
    let mut headers = HeaderMap::new();
    headers.append(
        USER_AGENT,
        HeaderValue::from_static("mangadex-api-rs 3.2.0"),
    );
    Client::builder().default_headers(headers).build().unwrap()
}
