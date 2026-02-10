pub mod commands;
pub mod config;
pub mod constants;
pub mod entity;
pub mod events;
pub mod floor;
pub mod grid;
pub mod physics;
pub mod plugin;
pub mod spawn;
pub mod state;
pub mod systems;
pub mod tile;
pub mod tile_components;

pub use commands::DungeonCommands;
pub use config::DungeonConfig;
pub use entity::{
    ChestEntity, CraftingStationEntity, DoorEntity, DungeonEntityMarker, MobEntity, NpcEntity,
    RockEntity, StairsEntity,
};
pub use floor::{FloorId, FloorSpec};
pub use constants::{ColliderConfig, FORGE_COLLIDER, MOB_COLLIDER, PLAYER_COLLIDER, STAIRS_COLLIDER, STATIC_COLLIDER};
pub use grid::EntitySize;
pub use physics::{attack_hitbox_layers, GameLayer};
pub use plugin::{DungeonBuilder, DungeonPlugin, DungeonRegistry, FloorMonsterCount, HasLocation, NoLocation, TiledWallCollider};
pub use spawn::{MobSpawnEntry, SpawnEntityType, SpawnEntry, SpawnTable};
pub use state::{DepthSorting, DungeonState, MovementConfig, TileWorldSize, TilemapInfo};
pub use systems::{
    handle_floor_transition, handle_mob_defeated, handle_player_collisions, handle_player_move,
    on_map_created, prepare_floor, stop_attacking_player, stop_player_when_idle, SpawnFloor,
};
pub use tile::{Tile, TileType};
pub use tile_components::{can_have_entity, can_spawn_player, is_door, is_solid};

pub use events::{
    ChestMined, CraftingStationInteraction, FloorReady, FloorTransition, MerchantInteraction,
    MineableEntityType, MiningResult, MoveResult, PlayerMoveIntent, RockMined,
};
