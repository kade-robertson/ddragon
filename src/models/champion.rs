use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::shared::Image;

#[derive(Serialize, Deserialize)]
pub(crate) struct ChampionWrapper {
    pub format: String,
    pub version: String,
    pub data: HashMap<String, Champion>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Champion {
    pub id: String,
    pub key: String,
    pub name: String,
    pub title: String,
    pub image: Image,
    pub skins: Vec<Skin>,
    pub lore: String,
    pub blurb: String,
    pub allytips: Vec<String>,
    pub enemytips: Vec<String>,
    pub tags: Vec<Tag>,
    pub partype: String,
    pub info: Info,
    pub stats: HashMap<String, f64>,
    pub spells: Vec<Spell>,
    pub passive: Passive,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Info {
    pub attack: i64,
    pub defense: i64,
    pub magic: i64,
    pub difficulty: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Passive {
    pub name: String,
    pub description: String,
    pub image: Image,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Skin {
    pub id: String,
    pub num: i64,
    pub name: String,
    pub chromas: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Spell {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tooltip: String,
    pub leveltip: Option<LevelTip>,
    pub maxrank: i64,
    pub cooldown: Vec<f64>,
    #[serde(rename = "cooldownBurn")]
    pub cooldown_burn: String,
    pub cost: Vec<i64>,
    #[serde(rename = "costBurn")]
    pub cost_burn: String,
    pub effect: Vec<Option<Vec<f64>>>,
    #[serde(rename = "effectBurn")]
    pub effect_burn: Vec<Option<String>>,
    #[serde(rename = "costType")]
    pub cost_type: String,
    pub maxammo: String,
    pub range: Vec<i64>,
    #[serde(rename = "rangeBurn")]
    pub range_burn: String,
    pub image: Image,
    pub resource: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct LevelTip {
    pub label: Vec<String>,
    pub effect: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tag {
    Assassin,
    Fighter,
    Mage,
    Marksman,
    Support,
    Tank,
}
