pub mod commands;
pub mod config;
pub mod entity;
pub mod floor;
pub mod generator;
pub mod grid;
pub mod layout;
pub mod layout_builder;
pub mod layouts;
pub mod plugin;
pub mod rendering;
pub mod room_patterns;
pub mod spawn;
pub mod spawn_rules;
pub mod state;
pub mod tile;
pub mod tmx;
pub mod tmx_tileset;
pub mod variant_strategy;

pub use commands::DungeonCommands;
pub use config::DungeonConfig;
pub use entity::{DungeonEntity, EntityRenderData};
pub use floor::{FloorId, FloorInstance, FloorSpec, FloorType, GeneratedFloor, WeightedFloorPool};
pub use generator::LayoutGenerator;
pub use grid::{GridOccupancy, GridPosition, GridSize};
pub use layout::DungeonLayout;
pub use layout_builder::LayoutBuilder;
pub use layouts::LayoutId;
pub use plugin::{DungeonBuilder, DungeonPlugin, DungeonRegistry};
pub use rendering::{resolve_tile, ResolvedTile, TileRenderer};
pub use spawn::{SpawnEntityType, SpawnEntry, SpawnTable};
pub use spawn_rules::{
    ChestSpawner, ComposedSpawnRules, CraftingStationSpawner, FixedPositionSpawner,
    GuaranteedMobSpawner, NpcSpawner, RockSpawner, SpawnRule, SpawnRuleKind, StairsSpawner,
    WeightedMobEntry, WeightedMobSpawner,
};
pub use room_patterns::{ComposedPattern, Rect, RoomPattern, RoomPatternKind};
pub use state::DungeonState;
pub use tile::{Tile, TileType};
pub use variant_strategy::{
    CheckerboardVariant, ClusteredVariant, PatternVariant, PercentageVariant, TileTypeVariant,
    UniformVariant, VariantStrategy, VariantStrategyKind,
};
pub use tmx::{parse_tmx, parse_tsx, TileProperties, Tileset, TmxError, TmxMap};
pub use tmx_tileset::{init_tmx_tileset_grid, TmxTilesetGrid};
