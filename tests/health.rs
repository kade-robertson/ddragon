#[cfg(any(feature = "sync", feature = "async"))]
use std::{env::temp_dir, fs::remove_dir_all, time::Instant};

#[cfg(feature = "sync")]
use ddragon::Client;

#[cfg(feature = "sync")]
#[test]
fn health_check() {
    let tempdir = temp_dir().join("ddragon-cache");
    let _ = remove_dir_all(&tempdir);

    let client = Client::new(tempdir.as_os_str().to_str().unwrap()).unwrap();

    let uncached_start = Instant::now();
    let challenges = client.challenges().unwrap();
    let champion = client.champion("MonkeyKing").unwrap();
    let champion_image = client.image_of(&champion).unwrap();
    let champion_sprite = client.sprite_of(&champion).unwrap();
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
    let tft_arenas = client.tft_arenas().unwrap();
    let tft_augments = client.tft_augments().unwrap();
    let tft_champions = client.tft_champions().unwrap();
    let tft_hero_augments = client.tft_hero_augments().unwrap();
    let tft_items = client.tft_items().unwrap();
    let tft_queues = client.tft_queues().unwrap();
    let tft_regalia = client.tft_regalia().unwrap();
    let tft_tacticians = client.tft_tacticians().unwrap();
    let tft_traits = client.tft_traits().unwrap();
    let uncached_duration = uncached_start.elapsed();

    let cached_start = Instant::now();
    let cached_challenges = client.challenges().unwrap();
    let cached_champion = client.champion("MonkeyKing").unwrap();
    let cached_champion_image = client.image_of(&cached_champion).unwrap();
    let cached_champion_sprite = client.sprite_of(&cached_champion).unwrap();
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
    let cached_tft_arenas = client.tft_arenas().unwrap();
    let cached_tft_augments = client.tft_augments().unwrap();
    let cached_tft_champions = client.tft_champions().unwrap();
    let cached_tft_hero_augments = client.tft_hero_augments().unwrap();
    let cached_tft_items = client.tft_items().unwrap();
    let cached_tft_queues = client.tft_queues().unwrap();
    let cached_tft_regalia = client.tft_regalia().unwrap();
    let cached_tft_tacticians = client.tft_tacticians().unwrap();
    let cached_tft_traits = client.tft_traits().unwrap();
    let cached_duration = cached_start.elapsed();

    println!();
    dbg!(uncached_duration);
    dbg!(cached_duration);

    assert!(cached_duration < uncached_duration);
    assert_eq!(challenges, cached_challenges);
    assert_eq!(champion, cached_champion);
    assert_eq!(champion_image, cached_champion_image);
    assert_eq!(champion_sprite, cached_champion_sprite);
    assert_eq!(champions, cached_champions);
    assert_eq!(champions_full, cached_champions_full);
    assert_eq!(items, cached_items);
    assert_eq!(maps, cached_maps);
    assert_eq!(mission_assets, cached_mission_assets);
    assert_eq!(profile_icons, cached_profile_icons);
    assert_eq!(runes, cached_runes);
    assert_eq!(spell_buffs, cached_spell_buffs);
    assert_eq!(summoner_spells, cached_summoner_spells);
    assert_eq!(translations, cached_translations);
    assert_eq!(tft_arenas, cached_tft_arenas);
    assert_eq!(tft_augments, cached_tft_augments);
    assert_eq!(tft_champions, cached_tft_champions);
    assert_eq!(tft_hero_augments, cached_tft_hero_augments);
    assert_eq!(tft_items, cached_tft_items);
    assert_eq!(tft_queues, cached_tft_queues);
    assert_eq!(tft_regalia, cached_tft_regalia);
    assert_eq!(tft_tacticians, cached_tft_tacticians);
    assert_eq!(tft_traits, cached_tft_traits);
}

#[cfg(feature = "async")]
use ddragon::AsyncClient;

#[cfg(feature = "async")]
use tokio_test::block_on;

#[cfg(feature = "async")]
#[test]
fn async_health_check() {
    let tempdir = temp_dir().join("ddragon-async-cache");
    let _ = remove_dir_all(&tempdir);

    let client = block_on(AsyncClient::new(tempdir.as_os_str().to_str().unwrap())).unwrap();

    let uncached_start = Instant::now();
    let challenges = block_on(client.challenges()).unwrap();
    let champion = block_on(client.champion("MonkeyKing")).unwrap();
    let champion_image = block_on(client.image_of(&champion)).unwrap();
    let champion_sprite = block_on(client.sprite_of(&champion)).unwrap();
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
    let tft_arenas = block_on(client.tft_arenas()).unwrap();
    let tft_augments = block_on(client.tft_augments()).unwrap();
    let tft_champions = block_on(client.tft_champions()).unwrap();
    let tft_hero_augments = block_on(client.tft_hero_augments()).unwrap();
    let tft_items = block_on(client.tft_items()).unwrap();
    let tft_queues = block_on(client.tft_queues()).unwrap();
    let tft_regalia = block_on(client.tft_regalia()).unwrap();
    let tft_tacticians = block_on(client.tft_tacticians()).unwrap();
    let tft_traits = block_on(client.tft_traits()).unwrap();
    let uncached_duration = uncached_start.elapsed();

    let cached_start = Instant::now();
    let cached_challenges = block_on(client.challenges()).unwrap();
    let cached_champion = block_on(client.champion("MonkeyKing")).unwrap();
    let cached_champion_image = block_on(client.image_of(&cached_champion)).unwrap();
    let cached_champion_sprite = block_on(client.sprite_of(&cached_champion)).unwrap();
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
    let cached_tft_arenas = block_on(client.tft_arenas()).unwrap();
    let cached_tft_augments = block_on(client.tft_augments()).unwrap();
    let cached_tft_champions = block_on(client.tft_champions()).unwrap();
    let cached_tft_hero_augments = block_on(client.tft_hero_augments()).unwrap();
    let cached_tft_items = block_on(client.tft_items()).unwrap();
    let cached_tft_queues = block_on(client.tft_queues()).unwrap();
    let cached_tft_regalia = block_on(client.tft_regalia()).unwrap();
    let cached_tft_tacticians = block_on(client.tft_tacticians()).unwrap();
    let cached_tft_traits = block_on(client.tft_traits()).unwrap();
    let cached_duration = cached_start.elapsed();

    println!();
    dbg!(uncached_duration);
    dbg!(cached_duration);

    assert!(cached_duration < uncached_duration);
    assert_eq!(challenges, cached_challenges);
    assert_eq!(champion, cached_champion);
    assert_eq!(champion_image, cached_champion_image);
    assert_eq!(champion_sprite, cached_champion_sprite);
    assert_eq!(champions, cached_champions);
    assert_eq!(champions_full, cached_champions_full);
    assert_eq!(items, cached_items);
    assert_eq!(maps, cached_maps);
    assert_eq!(mission_assets, cached_mission_assets);
    assert_eq!(profile_icons, cached_profile_icons);
    assert_eq!(runes, cached_runes);
    assert_eq!(spell_buffs, cached_spell_buffs);
    assert_eq!(summoner_spells, cached_summoner_spells);
    assert_eq!(translations, cached_translations);
    assert_eq!(tft_arenas, cached_tft_arenas);
    assert_eq!(tft_augments, cached_tft_augments);
    assert_eq!(tft_champions, cached_tft_champions);
    assert_eq!(tft_hero_augments, cached_tft_hero_augments);
    assert_eq!(tft_items, cached_tft_items);
    assert_eq!(tft_queues, cached_tft_queues);
    assert_eq!(tft_regalia, cached_tft_regalia);
    assert_eq!(tft_tacticians, cached_tft_tacticians);
    assert_eq!(tft_traits, cached_tft_traits);
}
