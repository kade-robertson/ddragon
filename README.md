# ddragon

[![latest version](https://img.shields.io/crates/v/ddragon?style=flat-square)](https://crates.io/crates/ddragon) [![health check status](https://img.shields.io/github/actions/workflow/status/kade-robertson/ddragon/health.yml?label=health&style=flat-square)](https://github.com/kade-robertson/ddragon/actions/workflows/health.yml) [![downloads of latest version](https://img.shields.io/crates/d/ddragon?style=flat-square)](https://crates.io/crates/ddragon) [![latest docs](https://img.shields.io/docsrs/ddragon?style=flat-square)](https://docs.rs/ddragon/latest/ddragon/)

Rust library for accessing the latest LoL patch's ddragon data.

- Full JSON deserialization via `serde_json`
- Provides a synchronous API by default
  - Local caching via `cacache-sync`
  - Accepts custom `ureq` agents (which can use the exposed cache middleware)
- Optionally, an asynchronous API can be used that maintains the same featureset
  - Local caching is handled by `http-cache-reqwest` rather than a custom middleware
  - Also accepts custom `reqwest` or `reqwest-middleware` clients
- Optionally, some useful functions to fetch and decode images, via `image`

## Usage

```rust
use ddragon::{cache_middleware::CacheMiddleware, DDragonClientError, DDragonClient};

fn main() -> Result<(), DDragonClientError> {
    let client = DDragonClient::new("/path/to/your/cache/dir")?;

    // If you want to use an existing agent
    let my_agent = ureq::AgentBuilder::new()
        .middleware(CacheMiddleware::new("/path/to/your/cache/dir"))
        .build();
    let client = DDragonClient::with_agent(my_agent)?;

    // See available options on the client and in the models folder.
    let champions = client.champions()?;
    let runes = client.runes()?;

    Ok(())
}
```

## Features

The following crate features are available:

- `sync` (on by default) enables the synchronous client.
  - Provides the `ddragon::client` and `ddragon::cache_middleware` module.
  - Provides the re-exported `ddragon::DDragonClient` client.
  - Adds `cacache-sync`, `url`, `thiserror`, and `ureq` with the `json` feature enabled as dependencies.
- `async` enables the asynchronous client.
  - Provides the `ddragon::async_client` module.
  - Provides the re-exported `ddragon::AsyncDDragonClient` client.
  - Adds `reqwest` with the `json` feature, `reqwest-middleware` and `http-cache-reqwest` as dependencies.
- `image` enables image fetching and caching.
  - Both clients will receive `image_of` and `sprite_of` for any model which implements `HasImage`.
  - Adds the `image` dependency.

To use the library with just the synchronous version, it should be as simple as adding any other dependency:

```toml
[dependencies]
ddragon = "<version>"
```

If you would also like to have the image fetching support, use:

```toml
[dependencies]
ddragon = { version = "<version>", features = ["image"] }
```

If you want the asynchronous client only, you probably don't want to pull in the dependencies related to the synchronous code, so you can do this:

```toml
[dependencies]
ddragon = { version = "<version>", default-features = false, features = ["async"] }
```

If you only want the DDragon models (none of the client code), you can use

```toml
[dependencies]
ddragon = { version = "<version>", default-features = false }
```

## Roadmap

- [x] Support all `.json` endpoints related to League of Legends
- [ ] Support endpoints related to Teamfight Tactics
- [x] Add additional helpers for obtaining image assets
- [x] Add an async API using `reqwest` as the backend
- [ ] Improve docs
