use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Image<T> {
    pub full: String,
    pub sprite: T,
    pub group: String,
    pub x: i64,
    pub y: i64,
    pub w: i64,
    pub h: i64,
}
