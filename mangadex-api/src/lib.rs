#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![cfg_attr(
    not(any(feature = "multi-thread", feature = "tokio-multi-thread")),
    allow(clippy::await_holding_refcell_ref)
)]

pub mod constants;
#[macro_use]
pub mod http_client;
pub mod v5;

#[cfg(feature = "utils")]
pub mod utils;

pub use constants::*;
pub use http_client::{HttpClient, HttpClientRef};
pub use v5::MangaDexClient;
