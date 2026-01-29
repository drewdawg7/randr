use bevy::prelude::*;

use crate::dungeon::events::{FloorReady, FloorTransition};
use crate::dungeon::{DungeonRegistry, DungeonState, FloorType};
use crate::location::LocationId;

/// Handles all floor transitions by pattern matching on the variant.
pub fn handle_floor_transition(
    mut events: EventReader<FloorTransition>,
    mut result_events: EventWriter<FloorReady>,
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

        state.load_floor_layout();

        let Some(layout) = state.layout.clone() else {
            continue;
        };

        let floor_type = state
            .current_floor()
            .map(|f| f.floor_type())
            .unwrap_or(FloorType::CaveFloor);

        result_events.send(FloorReady {
            layout,
            player_pos: state.player_pos,
            player_size: state.player_size,
            floor_type,
        });
    }
}
