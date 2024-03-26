use std::collections::HashSet;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SkillMod {
    // idk
    pub title: String,
    pub description: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SkillGem {
    pub item_id: u32,
    pub skill_id: u32,
    pub mods: Vec<SkillMod>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RawSkillGem {
    pub item_id: u32,
    pub skill_id: u32,
    pub compatable_reagents: Vec<Reagent>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Reagent {
    pub name: String,
}
