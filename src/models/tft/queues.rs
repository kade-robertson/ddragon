use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::models::shared::{has_image, Image};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Queues {
    pub version: String,
    pub data: HashMap<String, Queue>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Queue {
    pub id: String,
    pub name: String,
    #[serde(rename = "queueType")]
    pub queue_type: String,
    pub image: Image,
}

has_image!(Queue);
