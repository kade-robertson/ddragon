use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::shared::Image;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Maps {
    pub version: String,
    pub data: HashMap<String, Map>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Map {
    #[serde(rename = "MapName")]
    pub map_name: String,
    #[serde(rename = "MapId")]
    pub map_id: String,
    pub image: Image,
}
