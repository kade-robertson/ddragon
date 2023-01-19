# ddragon

Rust library for accessing the latest LoL patch's ddragon data.

- Full JSON deserialization via `serde_json`
- Local caching via `cacache`
- Accepts custom `ureq` agents

## Usage

```rust
use ureq;
use ddragon::DDragonClient;

fn main() -> Result<(), ddragon::client::DDragonClientError> {
    // Using caching, the preferred option.
    let client = DDragonClient::with_cache("/path/to/your/cache/dir")?;

    // If you want to use an existing agent
    let client = DDragonClient::with_agent(ureq::Agent::new(), Some("/path/to/your/cache/dir".to_owned()))?;

    // See available options on the client and in the models folder.
    let champions = client.champions()?:
    let runes = client.runes()?;

    Ok(())
}
```
