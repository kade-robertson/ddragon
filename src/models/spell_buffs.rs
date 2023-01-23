use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SpellBuffs {
    #[serde(rename = "spellBuffs")]
    pub spell_buffs: Vec<SpellBuff>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SpellBuff {
    pub id: i64,
    pub name: String,
}
