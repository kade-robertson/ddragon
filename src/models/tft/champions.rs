use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::models::shared::{Image, has_image};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Champions {
    pub version: String,
    pub data: HashMap<String, Champion>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Champion {
    pub id: String,
    pub name: String,
    pub tier: i64,
    pub image: Image,
}

has_image!(Champion);
