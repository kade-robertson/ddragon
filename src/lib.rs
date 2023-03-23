#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

/// Contains the structs that can be returned by the client API.
pub mod models;

#[cfg(any(feature = "sync", feature = "async-base"))]
mod error;

#[cfg(any(feature = "sync", feature = "async-base"))]
pub use error::ClientError;

#[cfg(feature = "sync")]
/// Contains the local file caching middleware used for `ureq`.
pub mod cache_middleware;

#[cfg(feature = "sync")]
/// Contains the sync version of the client API.
pub mod client;

#[cfg(feature = "sync")]
pub use client::Client;

#[cfg(feature = "sync")]
pub use client::ClientBuilder;

#[cfg(any(feature = "async", feature = "async-rustls"))]
/// Contains the async version of the client API.
pub mod async_client;

#[cfg(any(feature = "async", feature = "async-rustls"))]
pub use async_client::AsyncClient;

#[cfg(any(feature = "async", feature = "async-rustls"))]
pub use async_client::AsyncClientBuilder;
