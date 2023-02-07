use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::models::shared::{has_image, Image};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Augments {
    pub version: String,
    #[serde(rename = "augment-container")]
    pub augment_container: AugmentContainer,
    pub data: HashMap<String, Augment>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AugmentContainer {
    pub name: String,
    pub image: Image,
}

has_image!(AugmentContainer);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Augment {
    pub id: String,
    pub name: String,
    pub image: Image,
}

has_image!(Augment);
