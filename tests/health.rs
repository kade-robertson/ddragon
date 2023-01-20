use std::{env::temp_dir, fs::remove_dir_all, time::Instant};

use ddragon::DDragonClient;

#[test]
fn health_check() {
    let tempdir = temp_dir().join("ddragon-cache");
    let _ = remove_dir_all(&tempdir);

    eprintln!("Using {} for cache.", tempdir.to_string_lossy());
    let client = DDragonClient::new(tempdir.as_os_str().to_str().unwrap()).unwrap();

    let uncached_start = Instant::now();
    client.challenges().unwrap();
    client.champions().unwrap();
    client.items().unwrap();
    client.runes().unwrap();
    client.summoner_spells().unwrap();
    client.translations().unwrap();
    let uncached_duration = uncached_start.elapsed();

    let cached_start = Instant::now();
    client.challenges().unwrap();
    client.champions().unwrap();
    client.items().unwrap();
    client.runes().unwrap();
    client.summoner_spells().unwrap();
    client.translations().unwrap();
    let cached_duration = cached_start.elapsed();

    assert!(cached_duration < uncached_duration);
}
