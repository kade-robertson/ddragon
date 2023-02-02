use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::shared::Image;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct MissionAssets {
    pub version: String,
    pub data: HashMap<String, MissionAsset>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct MissionAsset {
    pub id: i64,
    pub image: Image,
}
