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
    client.champion("MonkeyKing").unwrap();
    client.champions().unwrap();
    client.champions_full().unwrap();
    client.items().unwrap();
    client.maps().unwrap();
    client.mission_assets().unwrap();
    client.profile_icons().unwrap();
    client.runes().unwrap();
    client.spell_buffs().unwrap();
    client.summoner_spells().unwrap();
    client.translations().unwrap();
    let uncached_duration = uncached_start.elapsed();

    let cached_start = Instant::now();
    client.challenges().unwrap();
    client.champion("MonkeyKing").unwrap();
    client.champions().unwrap();
    client.champions_full().unwrap();
    client.items().unwrap();
    client.maps().unwrap();
    client.mission_assets().unwrap();
    client.profile_icons().unwrap();
    client.runes().unwrap();
    client.spell_buffs().unwrap();
    client.summoner_spells().unwrap();
    client.translations().unwrap();
    let cached_duration = cached_start.elapsed();

    assert!(cached_duration < uncached_duration);
}

#[cfg(feature = "async")]
use ddragon::AsyncDDragonClient;

#[cfg(feature = "async")]
use tokio_test::block_on;

#[cfg(feature = "async")]
#[test]
fn async_health_check() {
    let tempdir = temp_dir().join("ddragon-async-cache");
    let _ = remove_dir_all(&tempdir);

    eprintln!("Using {} for cache.", tempdir.to_string_lossy());
    let client = block_on(AsyncDDragonClient::new(
        tempdir.as_os_str().to_str().unwrap(),
    ))
    .unwrap();

    let uncached_start = Instant::now();
    block_on(client.challenges()).unwrap();
    block_on(client.champion("MonkeyKing")).unwrap();
    block_on(client.champions()).unwrap();
    block_on(client.champions_full()).unwrap();
    block_on(client.items()).unwrap();
    block_on(client.maps()).unwrap();
    block_on(client.mission_assets()).unwrap();
    block_on(client.profile_icons()).unwrap();
    block_on(client.runes()).unwrap();
    block_on(client.spell_buffs()).unwrap();
    block_on(client.summoner_spells()).unwrap();
    block_on(client.translations()).unwrap();
    let uncached_duration = uncached_start.elapsed();

    let cached_start = Instant::now();
    block_on(client.challenges()).unwrap();
    block_on(client.champion("MonkeyKing")).unwrap();
    block_on(client.champions()).unwrap();
    block_on(client.champions_full()).unwrap();
    block_on(client.items()).unwrap();
    block_on(client.maps()).unwrap();
    block_on(client.mission_assets()).unwrap();
    block_on(client.profile_icons()).unwrap();
    block_on(client.runes()).unwrap();
    block_on(client.spell_buffs()).unwrap();
    block_on(client.summoner_spells()).unwrap();
    block_on(client.translations()).unwrap();
    let cached_duration = cached_start.elapsed();

    assert!(cached_duration < uncached_duration);
}
