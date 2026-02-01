pub mod commands;
pub mod config;
pub mod entity;
pub mod events;
pub mod floor;
pub mod grid;
pub mod layout;
pub mod layouts;
pub mod map;
pub mod plugin;
pub mod spawn;
pub mod state;
pub mod systems;
pub mod tile;
pub mod tile_components;
pub mod tiled;

pub use commands::DungeonCommands;
pub use config::DungeonConfig;
pub use entity::{DungeonEntity, DungeonEntityMarker, EntityRenderData};
pub use floor::{FloorId, FloorSpec, FloorType};
pub use grid::{GridOccupancy, GridPosition, GridSize};
pub use layout::DungeonLayout;
pub use layouts::LayoutId;
pub use map::{entity_spawn_positions, map_to_layout, player_spawn_positions};
pub use plugin::{DungeonBuilder, DungeonPlugin, DungeonRegistry, FloorMonsterCount};
pub use spawn::{SpawnEntityType, SpawnEntry, SpawnTable};
pub use state::DungeonState;
pub use systems::{
    handle_floor_transition, handle_mine_entity, handle_mob_defeated, handle_player_move,
    on_map_created, prepare_floor, FloorSpawnConfig, MobSpawnEntry, SpawnFloor,
};
pub use tile::{Tile, TileType};
pub use tile_components::{can_have_entity, can_spawn_player, is_door, is_solid};
pub use tiled::map_path;

pub use events::{
    CraftingStationInteraction, FloorReady, FloorTransition, MineEntity, MiningResult, MoveResult,
    NpcInteraction, PlayerMoveIntent,
};
