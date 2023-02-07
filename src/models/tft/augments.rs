use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::models::shared::{has_image, BasicDatum, Image};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Augments {
    pub version: String,
    #[serde(rename = "augment-container")]
    pub augment_container: AugmentContainer,
    pub data: HashMap<String, BasicDatum>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AugmentContainer {
    pub name: String,
    pub image: Image,
}

has_image!(AugmentContainer);
