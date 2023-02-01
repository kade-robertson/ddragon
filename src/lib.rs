#![doc = include_str!("../README.md")]

/// Contains the structs that can be returned by the client API.
pub mod models;

#[cfg(feature = "local-cache")]
/// Contains the local file caching middleware used for `ureq`.
pub mod cache_middleware;

#[cfg(feature = "sync")]
/// Contains the sync version of the client API.
pub mod client;

#[cfg(feature = "sync")]
pub use client::DDragonClient;

#[cfg(feature = "async")]
/// Contains the async version of the client API.
pub mod async_client;

#[cfg(feature = "async")]
pub use async_client::AsyncDDragonClient;
