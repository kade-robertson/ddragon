# ddragon

Rust library for accessing the latest LoL patch's ddragon data.

- Full JSON deserialization via `serde_json`
- Local caching via `cacache`
- Accepts custom `ureq` agents (which can use the exposed cache middleware)

## Usage

```rust
use ureq;
use ddragon::{DDragonClient, client::DDragonClientError, cache_middleware::CacheMiddleware};

fn main() -> Result<(), DDragonClientError> {
    // Using caching, the preferred option.
    let client = DDragonClient::with_cache("/path/to/your/cache/dir")?;

    // If you want to use an existing agent
    let my_agent = ureq::AgentBuilder::new()
        .middleware(CacheMiddleware::new("/path/to/your/cache/dir"))
        .build();
    let client = DDragonClient::with_agent(my_agent)?;

    // See available options on the client and in the models folder.
    let champions = client.champions()?:
    let runes = client.runes()?;

    Ok(())
}
```
