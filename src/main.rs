use combat::{CastType, DamageEffect, Effect, EffectType, Skill, SkillTargeting};

pub mod combat;
pub mod items;
fn main() {
    let fireball = Skill {
        skill_id: 0,
        name: "fireball".to_string(),
        description: "strike an area with a fireball doing {DAMAGE} damage.".to_string(),
        cooldown: 12.0,
        targeting_method: SkillTargeting::Point { range: 10.0 },
        cast_type: CastType::Charge {
            charge_duration: 4.0,
            stationary_cast: true,
        },
        effects: [Effect {
            area_of_effect: Some(2.0),
            varaint: EffectType::Damage(DamageEffect {
                true_damage: 0,
                melee_damage: 0,
                ranged_damage: 0,
                magic_damage: 20,
            }),
        }]
        .to_vec(),
    };
}
