use std::{collections::HashMap};

use rand::Rng;

use crate::{
    entities::mob::{enums::MobId, spec::specs::COW},
    registry::{RegistryDefaults, SpawnFromSpec},
    stats::{StatInstance, StatSheet, StatType},
};

use super::super::definition::Mob;
use super::definition::MobSpec;
use super::specs::{DRAGON, GOBLIN, SLIME};

impl SpawnFromSpec<MobId> for MobSpec {
    type Output = Mob;

    fn spawn_from_spec(kind: MobId, spec: &Self) -> Self::Output {
        let mut rng = rand::thread_rng();
        let hp_min = spec.max_health.start();
        let hp_max = spec.max_health.end();
        let hp_median = (hp_min + hp_max) as f32 / 2.0;
        let attack = rng.gen_range(spec.attack.clone());
        let defense = rng.gen_range(spec.defense.clone());
        let base_gold = rng.gen_range(spec.dropped_gold.clone());
        let max_hp = rng.gen_range(spec.max_health.clone());
        let hp = max_hp as f32;

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
                sheet.insert(StatType::Defense.instance(defense));
                sheet.insert(StatType::Health.instance(max_hp));
                sheet
            },
            loot_table: spec.loot.clone(),
            dropped_xp,
            death_processed: false,
        }
    }
}

impl RegistryDefaults<MobId> for MobSpec {
    fn defaults() -> impl IntoIterator<Item = (MobId, Self)> {
        [
            (MobId::Slime, SLIME.clone()),

            (MobId::Cow, COW.clone()),
            (MobId::Goblin, GOBLIN.clone()),
            (MobId::Dragon, DRAGON.clone()),
        ]
    }
}
