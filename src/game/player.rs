use bevy::prelude::*;

use crate::player::Player;

/// Event fired when the player takes damage
#[derive(Event, Debug, Clone)]
pub struct PlayerDamaged {
    pub amount: i32,
    pub current_hp: i32,
    pub max_hp: i32,
}

/// Event fired when the player is healed
#[derive(Event, Debug, Clone)]
pub struct PlayerHealed {
    pub amount: i32,
    pub current_hp: i32,
    pub max_hp: i32,
}

/// Event fired when the player levels up
#[derive(Event, Debug, Clone)]
pub struct PlayerLeveledUp {
    pub new_level: u32,
    pub old_level: u32,
}

/// Event fired when the player's gold changes
#[derive(Event, Debug, Clone)]
pub struct GoldChanged {
    pub amount: i32,
    pub new_total: i32,
}

/// Plugin that initializes the Player and registers player-related events
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Player>()
            .add_event::<PlayerDamaged>()
            .add_event::<PlayerHealed>()
            .add_event::<PlayerLeveledUp>()
            .add_event::<GoldChanged>();
    }
}
