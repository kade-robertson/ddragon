use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::shared::{has_image, Image};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Maps {
    pub version: String,
    pub data: HashMap<String, Map>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Map {
    #[serde(rename = "MapName")]
    pub map_name: String,
    #[serde(rename = "MapId")]
    pub map_id: String,
    pub image: Image,
}

has_image!(Map);
