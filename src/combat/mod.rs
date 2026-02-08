pub mod action_combat;
pub mod hitbox;
mod attack;
pub mod events;
pub mod plugin;
mod result;
mod system;
pub mod systems;
mod tests;

pub use hitbox::{AttackHitbox, AttackHitboxBundle, Attacking, HitEntities, HitboxLifetime};
pub use action_combat::ActionCombatPlugin;
pub use events::{
    DamageEntity, DealDamage, EntityDied, GoldGained, LootDropped, PlayerAttackMob, VictoryAchieved,
    XpGained,
};
pub use plugin::{ActiveCombat, CombatPlugin};

#[cfg(test)]
pub(crate) use system::{apply_defense, calculate_damage_reduction};
