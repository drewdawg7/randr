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
pub mod spawn_rules;
pub mod state;
pub mod systems;
pub mod tile;
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
pub use spawn_rules::{
    ChestSpawner, ComposedSpawnRules, CraftingStationSpawner, FixedPositionSpawner,
    GuaranteedMobSpawner, NpcSpawner, RockSpawner, SpawnRule, SpawnRuleKind, StairsSpawner,
    WeightedMobEntry, WeightedMobSpawner,
};
pub use state::DungeonState;
pub use systems::{
    handle_floor_transition, handle_mine_entity, handle_mob_defeated, handle_player_move,
    prepare_floor, SpawnFloor,
};
pub use tile::{Tile, TileType};
pub use tiled::map_path;

pub use events::{
    CraftingStationInteraction, FloorReady, FloorTransition, MineEntity, MiningResult, MoveResult,
    NpcInteraction, PlayerMoveIntent,
};
