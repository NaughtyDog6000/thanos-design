use std::collections::HashSet;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SkillMod {
    // idk
    pub title: String,
    pub description: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SkillGem {
    #[serde(rename = "_id")]
    pub item_id: ObjectId,
    pub skill_id: ObjectId,
    pub mods: Vec<SkillMod>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RawSkillGem {
    pub item_id: ObjectId,
    pub skill_id: ObjectId,
    pub compatable_reagents: Vec<Reagent>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Reagent {
    pub name: String,
}
