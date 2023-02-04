use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::shared::{has_image, Image};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ProfileIcons {
    pub version: String,
    pub data: HashMap<String, Datum>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Datum {
    pub id: Id,
    pub image: Image,
}

has_image!(Datum);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum Id {
    Integer(i64),
    String(String),
}
