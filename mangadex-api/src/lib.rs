#![doc = include_str!("../../README.md")]
#![forbid(unsafe_code)]

pub mod constants;
#[macro_use]
mod http_client;
pub mod v5;

#[cfg(feature = "utils")]
pub mod utils;

pub use constants::*;
pub use http_client::{HttpClient, HttpClientRef};
pub use v5::MangaDexClient;
