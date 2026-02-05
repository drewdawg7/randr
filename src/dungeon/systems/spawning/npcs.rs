use bevy::prelude::*;
use rand::Rng;

use crate::dungeon::spawn::SpawnTable;
use crate::dungeon::NpcEntity;

use super::context::{spawn_n_entities, SpawnContext};

pub fn spawn_npcs(
    commands: &mut Commands,
    config: &SpawnTable,
    available: &[Vec2],
    used: &mut Vec<Vec2>,
    ctx: &SpawnContext,
    rng: &mut impl Rng,
) {
    for (mob_id, count_range) in config.npc_spawns() {
        let count = rng.gen_range(count_range.clone());
        let mob_id = *mob_id;
        spawn_n_entities(commands, count, available, used, ctx, rng, |_| NpcEntity { mob_id });
    }

    for (mob_id, probability) in config.npc_chances() {
        if rng.gen_bool(*probability) {
            let mob_id = *mob_id;
            spawn_n_entities(commands, 1, available, used, ctx, rng, |_| NpcEntity { mob_id });
        }
    }
}
