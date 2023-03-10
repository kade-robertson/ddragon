use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::Champion;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ChampionsFull {
    pub format: String,
    pub version: String,
    pub data: HashMap<String, Champion>,
    pub keys: HashMap<String, String>,
}
