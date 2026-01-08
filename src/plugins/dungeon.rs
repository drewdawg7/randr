use bevy::prelude::*;

use crate::dungeon::{Dungeon, RoomType};

/// Resource wrapping the existing Dungeon system.
/// None when not in a dungeon, Some when inside one.
#[derive(Resource, Default)]
pub struct DungeonResource(pub Option<Dungeon>);

/// Event fired when the player enters a dungeon.
#[derive(Event)]
pub struct DungeonEntered {
    pub dungeon_name: String,
}

/// Event fired when the player exits a dungeon.
#[derive(Event)]
pub struct DungeonExited {
    pub dungeon_name: String,
}

/// Event fired when the player enters a room.
#[derive(Event)]
pub struct RoomEntered {
    pub room_type: RoomType,
    pub position: (i32, i32),
}

/// Event fired when a room is cleared of enemies.
#[derive(Event)]
pub struct RoomCleared {
    pub room_type: RoomType,
    pub position: (i32, i32),
}

/// Event fired when the boss is defeated.
#[derive(Event)]
pub struct BossDefeated {
    pub boss_name: String,
}

/// Plugin that integrates the existing dungeon system with Bevy ECS.
pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DungeonResource>()
            .add_event::<DungeonEntered>()
            .add_event::<DungeonExited>()
            .add_event::<RoomEntered>()
            .add_event::<RoomCleared>()
            .add_event::<BossDefeated>();
    }
}
