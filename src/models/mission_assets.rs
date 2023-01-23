use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::shared::Image;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MissionAssets {
    pub version: String,
    pub data: HashMap<String, MissionAsset>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MissionAsset {
    pub id: i64,
    pub image: Image,
}
