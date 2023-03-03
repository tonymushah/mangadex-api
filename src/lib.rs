#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

pub mod constants;
#[macro_use]
mod http_client;
pub mod v5;

pub use constants::*;
pub use http_client::{HttpClient, HttpClientRef};
pub use mangadex_api_types as types;
pub use mangadex_api_types::error::Result;
pub use v5::MangaDexClient;
