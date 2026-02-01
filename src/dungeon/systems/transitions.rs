use bevy::prelude::*;
use bevy_ecs_tiled::prelude::TilePos;

use crate::dungeon::events::FloorTransition;
use crate::dungeon::{DungeonRegistry, DungeonState, FloorType, GridSize, SpawnFloor};
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

        let floor_type = state
            .current_floor()
            .map(|f| f.floor_type())
            .unwrap_or(FloorType::CaveFloor);

        let layout_id = floor_type.layout_id(false);
        let (map_width, map_height) = layout_id.dimensions();

        state.player_pos = TilePos::new(map_width as u32 / 2, map_height as u32 / 2);
        state.player_size = GridSize::single();

        spawn_events.write(SpawnFloor {
            player_pos: state.player_pos,
            player_size: state.player_size,
            floor_type,
            map_width,
            map_height,
        });
    }
}
