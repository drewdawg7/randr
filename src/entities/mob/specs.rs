use std::collections::HashMap;

use rand::Rng;
use crate::{
    registry::{RegistryDefaults, SpawnFromSpec},
    stats::{StatInstance, StatSheet, StatType},
};

use super::{definition::{Mob, MobKind, MobSpec}, loot::loot_table_for};

impl SpawnFromSpec<MobKind> for MobSpec {
    type Output = Mob;
    fn spawn_from_spec(kind: MobKind, spec: &Self) -> Self::Output {
        let mut rng = rand::thread_rng();
        let attack  = rng.gen_range(spec.attack.clone());
        let gold    = rng.gen_range(spec.dropped_gold.clone());
        let max_hp  = rng.gen_range(spec.max_health.clone());
        Mob {
            spec: kind,
            name: spec.name,
            gold,
            stats: {
                let stats: HashMap<StatType, StatInstance> = HashMap::new();
                let mut sheet = StatSheet { stats };
                sheet.insert(StatType::Attack.instance(attack));
                sheet.insert(StatType::Health.instance(max_hp));
                sheet
            },
            loot_table: loot_table_for(kind),
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
                    max_health: 8..=12,
                    attack: 2..=4,
                    dropped_gold: 1..=3,
                },
            ),
            (
                MobKind::Goblin,
                MobSpec {
                    name: "Goblin",
                    max_health: 33..=41,
                    attack: 4..=6,
                    dropped_gold: 5..=7,
                },
            ),
        ]
    }
}
