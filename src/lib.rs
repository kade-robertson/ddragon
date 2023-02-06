#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

/// Contains the structs that can be returned by the client API.
pub mod models;

#[cfg(any(feature = "sync", feature = "async"))]
mod error;

#[cfg(any(feature = "sync", feature = "async"))]
pub use error::DDragonClientError;

#[cfg(feature = "sync")]
/// Contains the local file caching middleware used for `ureq`.
pub mod cache_middleware;

#[cfg(feature = "sync")]
/// Contains the sync version of the client API.
pub mod client;

#[cfg(feature = "sync")]
pub use client::Client;

#[cfg(feature = "async")]
/// Contains the async version of the client API.
pub mod async_client;

#[cfg(feature = "async")]
pub use async_client::AsyncDDragonClient;
