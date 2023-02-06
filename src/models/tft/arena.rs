use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::models::shared::Image;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Arenas {
    pub version: String,
    pub data: HashMap<String, Arena>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Arena {
    pub id: String,
    pub name: String,
    pub image: Image,
}
