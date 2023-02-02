#[cfg(any(feature = "sync", feature = "async"))]
use std::{env::temp_dir, fs::remove_dir_all, time::Instant};

#[cfg(feature = "sync")]
use ddragon::DDragonClient;

#[cfg(feature = "sync")]
#[test]
fn health_check() {
    let tempdir = temp_dir().join("ddragon-cache");
    let _ = remove_dir_all(&tempdir);

    eprintln!("Using {} for cache.", tempdir.to_string_lossy());
    let client = DDragonClient::new(tempdir.as_os_str().to_str().unwrap()).unwrap();

    let uncached_start = Instant::now();
    let challenges = client.challenges().unwrap();
    let champion = client.champion("MonkeyKing").unwrap();
    let champions = client.champions().unwrap();
    let champions_full = client.champions_full().unwrap();
    let items = client.items().unwrap();
    let maps = client.maps().unwrap();
    let mission_assets = client.mission_assets().unwrap();
    let profile_icons = client.profile_icons().unwrap();
    let runes = client.runes().unwrap();
    let spell_buffs = client.spell_buffs().unwrap();
    let summoner_spells = client.summoner_spells().unwrap();
    let translations = client.translations().unwrap();
    let uncached_duration = uncached_start.elapsed();

    let cached_start = Instant::now();
    let cached_challenges = client.challenges().unwrap();
    let cached_champion = client.champion("MonkeyKing").unwrap();
    let cached_champions = client.champions().unwrap();
    let cached_champions_full = client.champions_full().unwrap();
    let cached_items = client.items().unwrap();
    let cached_maps = client.maps().unwrap();
    let cached_mission_assets = client.mission_assets().unwrap();
    let cached_profile_icons = client.profile_icons().unwrap();
    let cached_runes = client.runes().unwrap();
    let cached_spell_buffs = client.spell_buffs().unwrap();
    let cached_summoner_spells = client.summoner_spells().unwrap();
    let cached_translations = client.translations().unwrap();
    let cached_duration = cached_start.elapsed();

    assert!(cached_duration < uncached_duration);
    assert_eq!(challenges, cached_challenges);
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
    let challenges = block_on(client.challenges()).unwrap();
    let champion = block_on(client.champion("MonkeyKing")).unwrap();
    let champions = block_on(client.champions()).unwrap();
    let champions_full = block_on(client.champions_full()).unwrap();
    let items = block_on(client.items()).unwrap();
    let maps = block_on(client.maps()).unwrap();
    let mission_assets = block_on(client.mission_assets()).unwrap();
    let profile_icons = block_on(client.profile_icons()).unwrap();
    let runes = block_on(client.runes()).unwrap();
    let spell_buffs = block_on(client.spell_buffs()).unwrap();
    let summoner_spells = block_on(client.summoner_spells()).unwrap();
    let translations = block_on(client.translations()).unwrap();
    let uncached_duration = uncached_start.elapsed();

    let cached_start = Instant::now();
    let cached_challenges = block_on(client.challenges()).unwrap();
    let cached_champion = block_on(client.champion("MonkeyKing")).unwrap();
    let cached_champions = block_on(client.champions()).unwrap();
    let cached_champions_full = block_on(client.champions_full()).unwrap();
    let cached_items = block_on(client.items()).unwrap();
    let cached_maps = block_on(client.maps()).unwrap();
    let cached_mission_assets = block_on(client.mission_assets()).unwrap();
    let cached_profile_icons = block_on(client.profile_icons()).unwrap();
    let cached_runes = block_on(client.runes()).unwrap();
    let cached_spell_buffs = block_on(client.spell_buffs()).unwrap();
    let cached_summoner_spells = block_on(client.summoner_spells()).unwrap();
    let cached_translations = block_on(client.translations()).unwrap();
    let cached_duration = cached_start.elapsed();

    assert!(cached_duration < uncached_duration);
    assert_eq!(challenges, cached_challenges);
}
