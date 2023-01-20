use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::shared::Image;

#[derive(Serialize, Deserialize, Clone)]
pub struct Items {
    pub version: String,
    pub data: HashMap<String, Item>,
    pub groups: Vec<Group>,
    pub tree: Vec<Tree>,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Gold {
    pub base: i64,
    pub total: i64,
    pub sell: i64,
    pub purchasable: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Rune {
    pub isrune: bool,
    pub tier: i64,
    #[serde(rename = "type")]
    pub rune_type: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub colloq: String,
    pub plaintext: String,
    pub into: Option<Vec<String>>,
    pub image: Image<ItemSprite>,
    pub gold: Gold,
    pub tags: Vec<String>,
    pub maps: HashMap<String, bool>,
    pub stats: HashMap<String, f64>,
    #[serde(rename = "inStore")]
    pub in_store: Option<bool>,
    pub from: Option<Vec<String>>,
    pub effect: Option<Effect>,
    pub depth: Option<i64>,
    pub consumed: Option<bool>,
    pub stacks: Option<i64>,
    #[serde(rename = "hideFromAll")]
    pub hide_from_all: Option<bool>,
    #[serde(rename = "consumeOnFull")]
    pub consume_on_full: Option<bool>,
    #[serde(rename = "requiredChampion")]
    pub required_champion: Option<String>,
    #[serde(rename = "requiredAlly")]
    pub required_ally: Option<String>,
    #[serde(rename = "specialRecipe")]
    pub special_recipe: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Effect {
    #[serde(rename = "Effect1Amount")]
    pub effect1_amount: String,
    #[serde(rename = "Effect2Amount")]
    pub effect2_amount: Option<String>,
    #[serde(rename = "Effect3Amount")]
    pub effect3_amount: Option<String>,
    #[serde(rename = "Effect4Amount")]
    pub effect4_amount: Option<String>,
    #[serde(rename = "Effect5Amount")]
    pub effect5_amount: Option<String>,
    #[serde(rename = "Effect6Amount")]
    pub effect6_amount: Option<String>,
    #[serde(rename = "Effect7Amount")]
    pub effect7_amount: Option<String>,
    #[serde(rename = "Effect8Amount")]
    pub effect8_amount: Option<String>,
    #[serde(rename = "Effect9Amount")]
    pub effect9_amount: Option<String>,
    #[serde(rename = "Effect10Amount")]
    pub effect10_amount: Option<String>,
    #[serde(rename = "Effect11Amount")]
    pub effect11_amount: Option<String>,
    #[serde(rename = "Effect12Amount")]
    pub effect12_amount: Option<String>,
    #[serde(rename = "Effect13Amount")]
    pub effect13_amount: Option<String>,
    #[serde(rename = "Effect14Amount")]
    pub effect14_amount: Option<String>,
    #[serde(rename = "Effect15Amount")]
    pub effect15_amount: Option<String>,
    #[serde(rename = "Effect16Amount")]
    pub effect16_amount: Option<String>,
    #[serde(rename = "Effect17Amount")]
    pub effect17_amount: Option<String>,
    #[serde(rename = "Effect18Amount")]
    pub effect18_amount: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Group {
    pub id: String,
    #[serde(rename = "MaxGroupOwnable")]
    pub max_group_ownable: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Tree {
    pub header: String,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum ItemSprite {
    #[serde(rename = "item0.png")]
    Item0,
    #[serde(rename = "item1.png")]
    Item1,
    #[serde(rename = "item2.png")]
    Item2,
}
