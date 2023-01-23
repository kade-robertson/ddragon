// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::ChampionsComplete;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: ChampionsComplete = serde_json::from_str(&json).unwrap();
// }

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::Champion;

#[derive(Serialize, Deserialize)]
pub struct ChampionsFull {
    pub format: String,
    pub version: String,
    pub data: HashMap<String, Champion>,
    pub keys: HashMap<String, String>,
}
