use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct SkillMod {
    // idk
    pub title: String,
    pub description: String,
}

#[derive(Clone, Debug)]
pub struct SkillGem {
    pub item_id: u32,
    pub skill_id: u32,
    pub mods: Vec<SkillMod>,
}

#[derive(Clone, Debug)]
pub struct RawSkillGem {
    pub item_id: u32,
    pub skill_id: u32,
    pub compatable_reagents: Vec<Reagent>,
}

#[derive(Clone, Debug)]
pub struct Reagent {
    pub name: String,
}
