use bevy::prelude::*;

use crate::dungeon::events::FloorTransition;
use crate::dungeon::plugin::FloorMonsterCount;
use crate::dungeon::{DungeonRegistry, DungeonState};
use crate::plugins::MobDefeated;

pub fn handle_mob_defeated(
    mut events: MessageReader<MobDefeated>,
    mut transition_events: MessageWriter<FloorTransition>,
    mut count: ResMut<FloorMonsterCount>,
    state: Res<DungeonState>,
    registry: Res<DungeonRegistry>,
) {
    for _ in events.read() {
        if count.0 > 0 {
            count.0 -= 1;
        }

        if count.0 == 0 && state.is_current_floor_final(&registry) {
            transition_events.write(FloorTransition::ReturnToHome);
        }
    }
}
