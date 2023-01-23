# ddragon

[![latest version](https://img.shields.io/crates/v/ddragon?style=flat-square)](https://crates.io/crates/ddragon) [![health check status](https://img.shields.io/github/actions/workflow/status/kade-robertson/ddragon/health.yml?label=health&style=flat-square)](https://github.com/kade-robertson/ddragon/actions/workflows/health.yml) [![downloads of latest version](https://img.shields.io/crates/d/ddragon?style=flat-square)](https://crates.io/crates/ddragon) [![latest docs](https://img.shields.io/docsrs/ddragon?style=flat-square)](https://docs.rs/ddragon/latest/ddragon/)

Rust library for accessing the latest LoL patch's ddragon data.

- Full JSON deserialization via `serde_json`
- Local caching via `cacache`
- Accepts custom `ureq` agents (which can use the exposed cache middleware)

## Usage

```rust
use ddragon::{cache_middleware::CacheMiddleware, client::DDragonClientError, DDragonClient};

fn main() -> Result<(), DDragonClientError> {
    // Using caching, the preferred option.
    // If you do not want caching enabled, disable the "local-cache" feature.
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

## Roadmap

- [x] Support all `.json` endpoints related to League of Legends
- [ ] Support endpoints related to Teamfight Tactics
- [ ] Add additional helpers for obtaining image assets
- [ ] Add an async API using `reqwest` as the backend
- [ ] Improve docs
