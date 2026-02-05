use bevy::prelude::*;

use crate::dungeon::events::FloorTransition;
use crate::dungeon::{DungeonRegistry, DungeonState, FloorId, SpawnFloor};
use crate::location::LocationId;

#[derive(Resource, Default)]
pub struct TransitionInProgress;

pub fn handle_floor_transition(
    mut commands: Commands,
    mut events: MessageReader<FloorTransition>,
    mut spawn_events: MessageWriter<SpawnFloor>,
    mut state: ResMut<DungeonState>,
    registry: Res<DungeonRegistry>,
    transition_in_progress: Option<Res<TransitionInProgress>>,
) {
    if transition_in_progress.is_some() {
        for _ in events.read() {}
        return;
    }

    for event in events.read() {
        commands.insert_resource(TransitionInProgress);
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

        let floor_id = state
            .current_floor()
            .unwrap_or(FloorId::HomeFloor);

        spawn_events.write(SpawnFloor { floor_id });
    }
}
