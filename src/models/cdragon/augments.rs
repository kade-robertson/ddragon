use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AugmentsResponse {
    pub augments: Vec<Augment>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
#[non_exhaustive]
pub enum AugmentRarity {
    Silver = 0,
    Gold = 1,
    Prismatic = 2,
    /// This is a terrible name, but this covers a mixture of:
    /// - Ability augments
    /// - Augments whose icons look like they'd be considered gold
    /// - Maybe other stuff
    ///
    /// I don't know why it's implemented like this.
    Misc = 4,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum DataValue {
    Single(f64),
    Scaling(Vec<f64>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Augment {
    pub api_name: String,
    pub data_values: HashMap<String, DataValue>,
    pub desc: String,
    pub icon_large: String,
    pub icon_small: String,
    pub id: i64,
    pub name: String,
    pub rarity: AugmentRarity,
    pub tooltip: String,
}
