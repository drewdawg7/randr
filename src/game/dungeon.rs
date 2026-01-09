use bevy::prelude::*;

use crate::dungeon::Dungeon;

/// Bevy Resource that wraps the existing Dungeon struct
#[derive(Resource, Debug)]
pub struct DungeonResource(pub Dungeon);

impl Default for DungeonResource {
    fn default() -> Self {
        Self(Dungeon::default())
    }
}

impl std::ops::Deref for DungeonResource {
    type Target = Dungeon;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for DungeonResource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Event fired when the player enters a new dungeon room
#[derive(Event, Debug, Clone)]
pub struct RoomEntered {
    pub room_type: crate::dungeon::RoomType,
    pub x: i32,
    pub y: i32,
}

/// Event fired when a room is cleared
#[derive(Event, Debug, Clone)]
pub struct RoomCleared {
    pub room_type: crate::dungeon::RoomType,
    pub x: i32,
    pub y: i32,
}

/// Event fired when the dungeon is completed
#[derive(Event, Debug, Clone)]
pub struct DungeonCompleted {
    pub dungeon_name: String,
    pub rooms_cleared: usize,
}

/// Plugin that initializes the DungeonResource and registers dungeon-related events
pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DungeonResource>()
            .add_event::<RoomEntered>()
            .add_event::<RoomCleared>()
            .add_event::<DungeonCompleted>();
    }
}
