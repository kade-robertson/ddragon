use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Image {
    pub full: String,
    pub sprite: String,
    pub group: String,
    pub x: i64,
    pub y: i64,
    pub w: i64,
    pub h: i64,
}

impl Image {
    pub fn image_path(&self) -> String {
        format!("{}/{}", self.group, self.full)
    }
    pub fn sprite_path(&self) -> String {
        format!("sprite/{}", self.sprite)
    }
}

pub trait HasImage {
    fn image_path(&self) -> String;
    fn sprite_path(&self) -> String;
}

macro_rules! has_image {
    ($s:ident) => {
        impl $crate::models::shared::HasImage for $s {
            fn image_path(&self) -> String {
                self.image.image_path()
            }
            fn sprite_path(&self) -> String {
                self.image.sprite_path()
            }
        }
    };
}
pub(crate) use has_image;
