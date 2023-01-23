#![doc = include_str!("../README.md")]

#[cfg(feature = "local-cache")]
/// Contains the local file caching middleware used for `ureq`.
pub mod cache_middleware;

/// Contains the client API.
pub mod client;

/// Contains the structs that can be returned by the client API.
pub mod models;

pub use client::DDragonClient;
