use bson::oid::ObjectId;
pub use combat::{CastType, DamageEffect, Effect, EffectType, Skill, SkillTargeting};
pub use skillgems::{SkillGem, SkillMod};

pub mod combat;
pub mod items;
pub mod skillgems;

pub fn main() {
    let mut skills: Vec<Skill> = Vec::new();
    let mut skill_gems: Vec<SkillGem> = Vec::new();

    skills.push(Skill {
        skill_id: ObjectId::new(),
        name: "fireball".to_string(),
        description: "strike an area with a fireball doing {DAMAGE} damage.".to_string(),
        cooldown: 12.0,
        targeting_method: SkillTargeting::Point { range: 10.0 },
        cast_type: CastType::Charge {
            charge_duration: 4.0,
            stationary_cast: true,
        },
        effects: [
            Effect {
                area_of_effect: Some(2.0),
                varaint: EffectType::Damage(DamageEffect {
                    true_damage: 0,
                    melee_damage: 0,
                    ranged_damage: 0,
                    magic_damage: 20,
                }),
            },
            Effect {
                area_of_effect: None,
                varaint: EffectType::BufDebuf,
            },
        ]
        .to_vec(),
    });

    skill_gems.push(SkillGem {
        skill_id: skills.first().unwrap().skill_id,
        item_id: ObjectId::new(),
        mods: [SkillMod {
            title: "Ignite".to_string(),
            description: "Burns targets for extra damage over time".to_string(),
        }]
        .to_vec(),
    });

    println!("printing Skill Gems:");
    for gem in skill_gems {
        println!("{:#?}", gem);
    }

    println!("printing Skills:");
    for skill in skills {
        println!("{:#?}", skill);
    }

}


pub fn example_skills() -> Vec<Skill> {
    let mut skills: Vec<Skill> = Vec::new();


    skills.push(Skill {
        skill_id: ObjectId::new(),
        name: "fireball".to_string(),
        description: "strike an area with a fireball doing {DAMAGE} damage.".to_string(),
        cooldown: 12.0,
        targeting_method: SkillTargeting::Point { range: 10.0 },
        cast_type: CastType::Charge {
            charge_duration: 4.0,
            stationary_cast: true,
        },
        effects: [
            Effect {
                area_of_effect: Some(2.0),
                varaint: EffectType::Damage(DamageEffect {
                    true_damage: 0,
                    melee_damage: 0,
                    ranged_damage: 0,
                    magic_damage: 20,
                }),
            },
            Effect {
                area_of_effect: None,
                varaint: EffectType::BufDebuf,
            },
        ]
        .to_vec(),
    });


    return skills;
}


pub fn example_skillgems() -> Vec<SkillGem> {
    let mut skillgems: Vec<SkillGem> = Vec::new();


    skillgems.push(SkillGem {
        skill_id: ObjectId::new(),
        item_id: ObjectId::new(),
        mods: [SkillMod {
            title: "Ignite".to_string(),
            description: "Burns targets for extra damage over time".to_string(),
        }]
        .to_vec(),
    });

    return skillgems;
}