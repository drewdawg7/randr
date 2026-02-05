pub mod action;
mod attack;
pub mod events;
pub mod plugin;
mod result;
mod system;
mod tests;

pub use action::{AttackHitbox, AttackHitboxBundle, Attacking, HitEntities, HitboxLifetime};
pub use events::{
    DamageEntity, DealDamage, EntityDied, GoldGained, LootDropped, PlayerAttackMob, VictoryAchieved,
    XpGained,
};
pub use plugin::{ActiveCombat, CombatPlugin};

#[cfg(test)]
pub(crate) use system::{apply_defense, calculate_damage_reduction};
