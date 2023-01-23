use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::shared::Image;

#[derive(Serialize, Deserialize)]
pub struct ProfileIcons {
    pub version: String,
    pub data: HashMap<String, Datum>,
}

#[derive(Serialize, Deserialize)]
pub struct Datum {
    pub id: Id,
    pub image: Image,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Id {
    Integer(i64),
    String(String),
}
