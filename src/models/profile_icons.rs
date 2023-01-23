use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::shared::Image;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProfileIcons {
    pub version: String,
    pub data: HashMap<String, Datum>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Datum {
    pub id: Id,
    pub image: Image,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Id {
    Integer(i64),
    String(String),
}
