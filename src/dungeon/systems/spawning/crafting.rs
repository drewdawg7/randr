use bevy::prelude::*;
use rand::Rng;

use crate::crafting_station::CraftingStationType;
use crate::dungeon::spawn::SpawnTable;
use crate::dungeon::CraftingStationEntity;

use super::context::{spawn_n_entities, SpawnContext};

pub fn spawn_crafting_stations(
    commands: &mut Commands,
    config: &SpawnTable,
    available: &[Vec2],
    used: &mut Vec<Vec2>,
    ctx: &SpawnContext,
    rng: &mut impl Rng,
) {
    let forge_count = if *config.forge().end() > 0 {
        rng.gen_range(config.forge().clone())
    } else if let Some(prob) = config.forge_chance() {
        if rng.gen_bool(prob) { 1 } else { 0 }
    } else {
        0
    };

    spawn_n_entities(commands, forge_count, available, used, ctx, rng, |_| {
        CraftingStationEntity {
            station_type: CraftingStationType::Forge,
        }
    });

    let anvil_count = if *config.anvil().end() > 0 {
        rng.gen_range(config.anvil().clone())
    } else if let Some(prob) = config.anvil_chance() {
        if rng.gen_bool(prob) { 1 } else { 0 }
    } else {
        0
    };

    spawn_n_entities(commands, anvil_count, available, used, ctx, rng, |_| {
        CraftingStationEntity {
            station_type: CraftingStationType::Anvil,
        }
    });
}
