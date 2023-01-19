use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::shared::Image;

#[derive(Serialize, Deserialize)]
pub struct Champions {
    pub format: String,
    pub version: String,
    pub data: HashMap<String, ChampionData>,
}

#[derive(Serialize, Deserialize)]
pub struct ChampionData {
    pub version: String,
    pub id: String,
    pub key: String,
    pub name: String,
    pub title: String,
    pub blurb: String,
    pub info: Info,
    pub image: Image<ChampionSprite>,
    pub tags: Vec<Tag>,
    pub partype: String,
    pub stats: HashMap<String, f64>,
}

#[derive(Serialize, Deserialize)]
pub struct Info {
    pub attack: i64,
    pub defense: i64,
    pub magic: i64,
    pub difficulty: i64,
}

#[derive(Serialize, Deserialize)]
pub enum ChampionSprite {
    #[serde(rename = "champion0.png")]
    Champion0,
    #[serde(rename = "champion1.png")]
    Champion1,
    #[serde(rename = "champion2.png")]
    Champion2,
    #[serde(rename = "champion3.png")]
    Champion3,
    #[serde(rename = "champion4.png")]
    Champion4,
    #[serde(rename = "champion5.png")]
    Champion5,
}

#[derive(Serialize, Deserialize)]
pub enum Tag {
    Assassin,
    Fighter,
    Mage,
    Marksman,
    Support,
    Tank,
}
