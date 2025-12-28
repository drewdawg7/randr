use std::collections::HashMap;

use crate::{
    registry::{RegistryDefaults, SpawnFromSpec},
    stats::{StatInstance, StatSheet, StatType},
};

use super::{definition::{Mob, MobKind, MobSpec}, loot::loot_table_for};

impl SpawnFromSpec<MobKind> for MobSpec {
    type Output = Mob;
    fn spawn_from_spec(kind: MobKind, spec: &Self) -> Self::Output {
        Mob {
            spec: kind,
            name: spec.name,
            stats: {
                let mut stats: HashMap<StatType, StatInstance> = HashMap::new();
                stats.insert(
                    StatType::Attack,
                    StatInstance {
                        stat_type: StatType::Attack,
                        current_value: spec.attack,
                        max_value: spec.attack,
                    },
                );

                stats.insert(
                    StatType::Health,
                    StatInstance {
                        stat_type: StatType::Health,
                        current_value: spec.max_health,
                        max_value: spec.max_health,
                    },
                );
                StatSheet { stats }
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
                    attack: 2,
                },
            ),
            (
                MobKind::Goblin,
                MobSpec {
                    name: "Goblin",
                    max_health: 45,
                    attack: 4,
                },
            ),
        ]
    }
}
