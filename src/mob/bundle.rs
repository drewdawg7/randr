//! Bevy bundles for spawning mob entities with ECS components.

use bevy::prelude::*;
use rand::Rng;

use super::components::{CombatStats, DeathProcessed, GoldReward, Health, MobLootTable, MobMarker, XpReward};
use super::MobId;

/// Bundle containing all combat-related components for a mob entity.
///
/// Spawn this bundle on dungeon mob entities to enable ECS-based combat.
/// The components are populated from the MobId's spec with randomized values.
#[derive(Bundle)]
pub struct MobCombatBundle {
    pub marker: MobMarker,
    pub health: Health,
    pub combat_stats: CombatStats,
    pub gold: GoldReward,
    pub xp: XpReward,
    pub loot: MobLootTable,
    pub death_processed: DeathProcessed,
}

impl MobCombatBundle {
    /// Create a new combat bundle from a MobId.
    /// Stats are randomized within the spec's ranges.
    pub fn from_mob_id(mob_id: MobId) -> Self {
        let spec = mob_id.spec();
        let mut rng = rand::thread_rng();

        // Calculate stats with the same logic as Mob::spawn
        let hp_min = spec.max_health.start();
        let hp_max = spec.max_health.end();
        let hp_median = (hp_min + hp_max) as f32 / 2.0;
        let max_hp = rng.gen_range(*spec.max_health.start()..=*spec.max_health.end());
        let hp = max_hp as f32;

        let attack = rng.gen_range(*spec.attack.start()..=*spec.attack.end());
        let defense = rng.gen_range(*spec.defense.start()..=*spec.defense.end());
        let base_gold = rng.gen_range(*spec.dropped_gold.start()..=*spec.dropped_gold.end());

        // XP and gold bonus based on HP roll
        let excess_ratio = if hp > hp_median {
            (hp - hp_median) / (*hp_max as f32 - hp_median)
        } else {
            0.0
        };
        let base_xp = rng.gen_range(*spec.dropped_xp.start()..=*spec.dropped_xp.end());
        let bonus_multiplier = 1.0 + excess_ratio * 0.5;
        let dropped_xp = (base_xp as f32 * bonus_multiplier).round() as i32;
        let gold = (base_gold as f32 * bonus_multiplier).round() as i32;

        Self {
            marker: MobMarker(mob_id),
            health: Health::new(max_hp),
            combat_stats: CombatStats { attack, defense },
            gold: GoldReward(gold),
            xp: XpReward(dropped_xp),
            loot: MobLootTable(spec.loot.clone()),
            death_processed: DeathProcessed::default(),
        }
    }
}
