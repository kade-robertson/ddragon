use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::models::shared::{has_image, Image};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Tacticians {
    pub version: String,
    pub data: HashMap<String, Tactician>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Tactician {
    pub id: String,
    pub tier: String,
    pub name: String,
    pub image: Image,
}

has_image!(Tactician);
