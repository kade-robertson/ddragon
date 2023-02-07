use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::models::shared::BasicDatum;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Traits {
    pub version: String,
    pub data: HashMap<String, BasicDatum>,
}
