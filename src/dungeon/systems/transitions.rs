use bevy::prelude::*;

use crate::dungeon::events::FloorTransition;
use crate::dungeon::constants::DEFAULT_TILE_SIZE;
use crate::dungeon::{DungeonRegistry, DungeonState, EntitySize, LayoutId, SpawnFloor, TileWorldSize};
use crate::location::LocationId;

pub fn handle_floor_transition(
    mut commands: Commands,
    mut events: MessageReader<FloorTransition>,
    mut spawn_events: MessageWriter<SpawnFloor>,
    mut state: ResMut<DungeonState>,
    registry: Res<DungeonRegistry>,
    tile_world_size: Option<Res<TileWorldSize>>,
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
        let (map_width, map_height) = layout_id.dimensions();

        let tile_size = tile_world_size.as_ref().map(|t| t.0).unwrap_or(DEFAULT_TILE_SIZE);
        let center_x = (map_width as f32 / 2.0) * tile_size;
        let center_y = (map_height as f32 / 2.0) * tile_size;
        state.player_pos = Vec2::new(center_x, center_y);
        state.player_size = EntitySize::new(tile_size, tile_size);

        spawn_events.write(SpawnFloor {
            player_pos: state.player_pos,
            player_size: state.player_size,
            layout_id,
            map_width,
            map_height,
        });
    }
}
