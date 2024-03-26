// skills are something that the user actively "casts" or activates

#[derive(Debug)]
pub struct Skill {
    pub skill_id: u32,
    pub name: String,
    pub description: String,
    pub cooldown: f32, // seconds

    pub targeting_method: SkillTargeting,
    pub cast_type: CastType,

    // things the skill does (damages an entitiy, applies a slow, poisons them etc)
    pub effects: Vec<Effect>,
}

#[derive(Debug)]
pub enum SkillTargeting {
    Point {
        range: f32,
    }, // a position in the world
    Entity {
        range: f32,
        can_target_friendly: bool,
        can_target_enemies: bool,
        can_target_self: bool,
    }, // casting on something (player, enemy, structure)
    None, // press button to cast / targets self
}

#[derive(Debug)]
pub enum CastType {
    Instant,
    Charge {
        charge_duration: f32,  // how long from cast start to effects being fired
        stationary_cast: bool, // if the player must be still while casting
    },
    // Toggle, //debateable if should be included
}

#[derive(Clone, Copy, Debug)]
pub enum EffectType {
    Damage(DamageEffect), //damage types and stats
    DOT(DamageOverTimeEffect),
    BufDebuf,                 //+20% haste, rage etc (temporary changes in power)
    Healing(HealEffect),      // instant/over time, amount, percent/aboslute
    Movement(MovementEffect), // teleports the player or dashes to a position etc
}

#[derive(Clone, Copy, Debug)]
pub struct Effect {
    pub area_of_effect: Option<f32>, // for AOE / spash damage Effects
    pub varaint: EffectType,
}

#[derive(Clone, Copy, Debug)]
pub struct DamageEffect {
    pub true_damage: u32, // damage that ignores all resistances
    pub melee_damage: u32,
    pub ranged_damage: u32,
    pub magic_damage: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct HealEffect {}

#[derive(Clone, Copy, Debug)]
pub struct MovementEffect {
    pub target_position: (f32, f32, f32), // where to move towards
    pub movement_duration: f32,           // how long to go from A->B (could be changed to speed)
    pub untargetable: bool, // if the user should be untargetable during this (dodge roll from darksouls)
    pub can_travel_unwalkable: bool, // if the path can go over an otherwise un-walkable surface (void/dashing over lava)
}

#[derive(Clone, Copy, Debug)]
pub struct DamageOverTimeEffect {
    pub damage_type: DamageOverTimeType,
    pub duration: f32,
    pub damage_per_tick: u32,
}

#[derive(Clone, Copy, Debug)]
pub enum DamageOverTimeType {
    Poison,
    Burn,
    Freeze,
}
