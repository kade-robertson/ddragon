use serde::{Deserialize, Serialize};

use crate::models::shared::{has_image, Image};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Regalia {
    pub version: String,
    pub data: RegaliaData,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RegaliaData {
    #[serde(rename = "RANKED_TFT")]
    pub ranked_tft: RankedTft,
    #[serde(rename = "RANKED_TFT_DOUBLE_UP")]
    pub ranked_tft_double_up: RankedTft,
    #[serde(rename = "RANKED_TFT_TURBO")]
    pub ranked_tft_turbo: RankedTftTurbo,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RankedTft {
    #[serde(rename = "Bronze")]
    pub bronze: RankImage,
    #[serde(rename = "Challenger")]
    pub challenger: RankImage,
    #[serde(rename = "Diamond")]
    pub diamond: RankImage,
    #[serde(rename = "Gold")]
    pub gold: RankImage,
    #[serde(rename = "Grandmaster")]
    pub grandmaster: RankImage,
    #[serde(rename = "Iron")]
    pub iron: RankImage,
    #[serde(rename = "Master")]
    pub master: RankImage,
    #[serde(rename = "Platinum")]
    pub platinum: RankImage,
    #[serde(rename = "Provisional")]
    pub provisional: RankImage,
    #[serde(rename = "Silver")]
    pub silver: RankImage,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RankedTftTurbo {
    #[serde(rename = "Blue")]
    pub blue: RankImage,
    #[serde(rename = "Gray")]
    pub gray: RankImage,
    #[serde(rename = "Green")]
    pub green: RankImage,
    #[serde(rename = "Hyper")]
    pub hyper: RankImage,
    #[serde(rename = "Purple")]
    pub purple: RankImage,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RankImage {
    pub image: Image,
}

has_image!(RankImage);
