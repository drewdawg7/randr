use bevy::prelude::*;

use crate::dungeon::events::FloorTransition;
use crate::dungeon::{DungeonRegistry, DungeonState, FloorType, SpawnFloor};
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

        let Some((_, spawn_config)) = state.load_floor_layout() else {
            continue;
        };
        commands.insert_resource(spawn_config);

        let Some(layout) = state.layout.clone() else {
            continue;
        };

        let floor_type = state
            .current_floor()
            .map(|f| f.floor_type())
            .unwrap_or(FloorType::CaveFloor);

        spawn_events.write(SpawnFloor {
            layout,
            player_pos: state.player_pos,
            player_size: state.player_size,
            floor_type,
        });
    }
}
