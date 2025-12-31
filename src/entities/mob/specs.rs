use std::collections::HashMap;

use rand::Rng;
use crate::{
    entities::mob::enums::MobQuality, registry::{RegistryDefaults, SpawnFromSpec}, stats::{StatInstance, StatSheet, StatType}
};

use super::{definition::{Mob, MobKind, MobSpec}, loot::loot_table_for};

impl SpawnFromSpec<MobKind> for MobSpec {
    type Output = Mob;
    fn spawn_from_spec(kind: MobKind, spec: &Self) -> Self::Output {
        let mut rng   = rand::thread_rng();
        let hp_min    = spec.max_health.start();
        let hp_max    = spec.max_health.end();
        let hp_median = (hp_min + hp_max) as f32 / 2.0;
        let attack    = rng.gen_range(spec.attack.clone());
        let base_gold = rng.gen_range(spec.dropped_gold.clone());
        let max_hp    = rng.gen_range(spec.max_health.clone());
        let hp        = max_hp as f32;

        let excess_ratio = if hp > hp_median {
            (hp - hp_median) / (*hp_max as f32 - hp_median)
        } else {
            0.0
        };
        let base_xp = rng.gen_range(spec.dropped_xp.clone());
        let bonus_multiplier = 1.0 + excess_ratio * 0.5;
        let dropped_xp = (base_xp as f32 * bonus_multiplier).round() as i32;
        let gold = (base_gold as f32 * bonus_multiplier).round() as i32;
        Mob {
            spec: kind,
            name: spec.name,
            quality: spec.quality.clone(),
            gold,
            stats: {
                let stats: HashMap<StatType, StatInstance> = HashMap::new();
                let mut sheet = StatSheet { stats };
                sheet.insert(StatType::Attack.instance(attack));
                sheet.insert(StatType::Health.instance(max_hp));
                sheet
            },
            loot_table: loot_table_for(kind),
            dropped_xp 
        }
    }
}

impl RegistryDefaults<MobKind> for MobSpec {
    fn defaults() -> impl IntoIterator<Item = (MobKind, MobSpec)> {
        [
            (
                MobKind::Slime,
                MobSpec {
                    name: "Slime",
                    quality: MobQuality::Normal,
                    max_health: 8..=12,
                    attack: 2..=4,
                    dropped_gold: 1..=3,
                    dropped_xp: 5..=9,
                },
            ),
            (
                MobKind::Goblin,
                MobSpec {
                    name: "Goblin",
                    quality: MobQuality::Normal,
                    max_health: 33..=41,
                    attack: 4..=6,
                    dropped_gold: 5..=7,
                    dropped_xp: 13..=20
                }
            ),
            (
                MobKind::Dragon,
                MobSpec {
                    name: "Dragon",
                    quality: MobQuality::Boss,
                    max_health: 500..=700,
                    attack: 25..=30,
                    dropped_gold: 250..=350,
                    dropped_xp: 500..=750,
                },
            ),
        ]
    }
}
