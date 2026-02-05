use bevy::prelude::*;
use rand::Rng;

use crate::dungeon::spawn::{MobSpawnEntry, SpawnTable};
use crate::dungeon::MobEntity;

use super::context::{spawn_n_entities, SpawnContext};

pub fn spawn_mobs(
    commands: &mut Commands,
    config: &SpawnTable,
    available: &[Vec2],
    used: &mut Vec<Vec2>,
    ctx: &SpawnContext,
    rng: &mut impl Rng,
) {
    for (mob_id, count) in config.guaranteed_mobs() {
        let mob_id = *mob_id;
        spawn_n_entities(commands, *count, available, used, ctx, rng, |_| MobEntity { mob_id });
    }

    let weighted_mobs = config.weighted_mobs();
    if weighted_mobs.is_empty() || *config.mob_count().end() == 0 {
        return;
    }

    let total_weight: u32 = weighted_mobs.iter().map(|e| e.weight).sum();
    if total_weight == 0 {
        return;
    }

    let count = rng.gen_range(config.mob_count().clone());

    spawn_n_entities(commands, count, available, used, ctx, rng, |rng| {
        let Some(entry) = weighted_select(&weighted_mobs, total_weight, rng) else {
            return MobEntity { mob_id: weighted_mobs[0].mob_id };
        };
        MobEntity { mob_id: entry.mob_id }
    });
}

fn weighted_select<'a>(
    entries: &'a [MobSpawnEntry],
    total_weight: u32,
    rng: &mut impl Rng,
) -> Option<&'a MobSpawnEntry> {
    if entries.is_empty() {
        return None;
    }

    let roll = rng.gen_range(0..total_weight);
    let mut cumulative = 0;

    for entry in entries {
        cumulative += entry.weight;
        if roll < cumulative {
            return Some(entry);
        }
    }

    entries.first()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mob::MobId;

    #[test]
    fn weighted_select_returns_entry() {
        let entries = vec![
            MobSpawnEntry { mob_id: MobId::Goblin, weight: 5 },
            MobSpawnEntry { mob_id: MobId::Slime, weight: 3 },
        ];

        let mut rng = rand::thread_rng();
        let result = weighted_select(&entries, 8, &mut rng);
        assert!(result.is_some());
    }

    #[test]
    fn weighted_select_empty_returns_none() {
        let entries: Vec<MobSpawnEntry> = vec![];
        let mut rng = rand::thread_rng();
        let result = weighted_select(&entries, 0, &mut rng);
        assert!(result.is_none());
    }
}
