use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{
    champion::{Info, Tag},
    shared::{has_image, Image},
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Champions {
    pub format: String,
    pub version: String,
    pub data: HashMap<String, ChampionShort>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ChampionShort {
    pub version: String,
    pub id: String,
    pub key: String,
    pub name: String,
    pub title: String,
    pub blurb: String,
    pub info: Info,
    pub image: Image,
    pub tags: Vec<Tag>,
    pub partype: String,
    pub stats: HashMap<String, f64>,
}

has_image!(ChampionShort);
