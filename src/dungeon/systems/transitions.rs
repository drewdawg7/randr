use bevy::prelude::*;

use crate::dungeon::events::FloorTransition;
use crate::dungeon::{DungeonRegistry, DungeonState, LayoutId, SpawnFloor};
use crate::location::LocationId;

pub fn handle_floor_transition(
    mut commands: Commands,
    mut events: MessageReader<FloorTransition>,
    mut spawn_events: MessageWriter<SpawnFloor>,
    mut state: ResMut<DungeonState>,
    registry: Res<DungeonRegistry>,
) {
    for event in events.read() {
        match event {
            FloorTransition::AdvanceFloor => {
                state.advance_floor(&registry);
            }
            FloorTransition::EnterDoor => {
                state.exit_dungeon();
                state.enter_dungeon(LocationId::MainDungeon, &registry);
            }
            FloorTransition::ReturnToHome => {
                state.reset_dungeon();
                state.exit_dungeon();
                state.enter_dungeon(LocationId::Home, &registry);
            }
        }

        let Some(spawn_config) = state.get_spawn_config() else {
            continue;
        };
        commands.insert_resource(spawn_config);

        let layout_id = state
            .current_floor()
            .map(|f| f.layout_id())
            .unwrap_or(LayoutId::CaveFloor);

        spawn_events.write(SpawnFloor { layout_id });
    }
}
