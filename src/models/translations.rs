use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Translations {
    pub version: String,
    pub data: HashMap<String, String>,
}
