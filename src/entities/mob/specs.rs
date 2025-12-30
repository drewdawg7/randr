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
        let attack = rng.gen_range(spec.attack.clone());
        Mob {
            spec: kind,
            name: spec.name,
            stats: {
                let stats: HashMap<StatType, StatInstance> = HashMap::new();
                let mut sheet = StatSheet { stats };
                sheet.insert(StatType::Attack.instance(attack));
                sheet.insert(StatType::Health.instance(spec.max_health));
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
                    max_health: 10,
                    attack: 2..=4,
                },
            ),
            (
                MobKind::Goblin,
                MobSpec {
                    name: "Goblin",
                    max_health: 45,
                    attack: 4..=6,
                },
            ),
        ]
    }
}
