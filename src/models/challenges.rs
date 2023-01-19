use serde::{Deserialize, Serialize};

pub type Challenges = Vec<Challenge>;

#[derive(Serialize, Deserialize)]
pub struct Challenge {
    pub id: i64,
    pub name: String,
    pub description: String,
    #[serde(rename = "shortDescription")]
    pub short_description: String,
    #[serde(rename = "hasLeaderboard")]
    pub has_leaderboard: bool,
    #[serde(rename = "levelToIconPath")]
    pub level_to_icon_path: LevelToIconPath,
    pub thresholds: Thresholds,
}

#[derive(Serialize, Deserialize)]
pub struct LevelToIconPath {
    #[serde(rename = "IRON")]
    pub iron: Option<String>,
    #[serde(rename = "BRONZE")]
    pub bronze: Option<String>,
    #[serde(rename = "SILVER")]
    pub silver: Option<String>,
    #[serde(rename = "GOLD")]
    pub gold: Option<String>,
    #[serde(rename = "PLATINUM")]
    pub platinum: Option<String>,
    #[serde(rename = "DIAMOND")]
    pub diamond: Option<String>,
    #[serde(rename = "MASTER")]
    pub master: Option<String>,
    #[serde(rename = "GRANDMASTER")]
    pub grandmaster: Option<String>,
    #[serde(rename = "CHALLENGER")]
    pub challenger: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Thresholds {
    #[serde(rename = "IRON")]
    pub iron: Option<RankReward>,
    #[serde(rename = "BRONZE")]
    pub bronze: Option<RankReward>,
    #[serde(rename = "SILVER")]
    pub silver: Option<RankReward>,
    #[serde(rename = "GOLD")]
    pub gold: Option<RankReward>,
    #[serde(rename = "PLATINUM")]
    pub platinum: Option<RankReward>,
    #[serde(rename = "DIAMOND")]
    pub diamond: Option<RankReward>,
    #[serde(rename = "MASTER")]
    pub master: Option<RankReward>,
    #[serde(rename = "GRANDMASTER")]
    pub grandmaster: Option<RankReward>,
    #[serde(rename = "CHALLENGER")]
    pub challenger: Option<RankReward>,
}

#[derive(Serialize, Deserialize)]
pub struct RankReward {
    pub value: i64,
    pub rewards: Option<Vec<RewardDetails>>,
}

#[derive(Serialize, Deserialize)]
pub struct RewardDetails {
    pub category: String,
    pub quantity: i64,
    pub title: String,
}
