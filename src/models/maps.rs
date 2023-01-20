use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Maps {
    #[serde(rename = "type")]
    pub maps_type: String,
    pub version: String,
    pub data: HashMap<String, Map>,
}

#[derive(Serialize, Deserialize)]
pub struct Map {
    #[serde(rename = "MapName")]
    pub map_name: String,
    #[serde(rename = "MapId")]
    pub map_id: String,
    pub image: Image,
}

#[derive(Serialize, Deserialize)]
pub struct Image {
    pub full: String,
    pub sprite: String,
    pub group: String,
    pub x: i64,
    pub y: i64,
    pub w: i64,
    pub h: i64,
}
