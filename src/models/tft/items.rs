use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::models::shared::BasicDatum;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Items {
    pub version: String,
    pub data: HashMap<String, BasicDatum>,
}
