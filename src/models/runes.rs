use serde::{Deserialize, Serialize};
pub type Runes = Vec<Rune>;

#[derive(Serialize, Deserialize, Clone)]
pub struct Rune {
    pub id: i64,
    pub key: String,
    pub icon: String,
    pub name: String,
    pub slots: Vec<Slot>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Slot {
    pub runes: Vec<RuneElement>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RuneElement {
    pub id: i64,
    pub key: String,
    pub icon: String,
    pub name: String,
    #[serde(rename = "shortDesc")]
    pub short_desc: String,
    #[serde(rename = "longDesc")]
    pub long_desc: String,
}
