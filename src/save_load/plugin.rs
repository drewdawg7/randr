use bevy::prelude::*;

use crate::game::{DungeonResource, Player, Storage};
use super::system::{load_game, save_game, save_exists};

/// Event to trigger saving the game
#[derive(Event, Debug, Clone)]
pub struct SaveGameEvent;

/// Event to trigger loading the game
#[derive(Event, Debug, Clone)]
pub struct LoadGameEvent;

/// Event sent after game is successfully loaded
#[derive(Event, Debug, Clone)]
pub struct GameLoaded {
    pub player_name: String,
    pub player_level: i32,
    pub player_gold: i32,
}

/// Event sent after game is successfully saved
#[derive(Event, Debug, Clone)]
pub struct GameSaved {
    pub player_name: String,
    pub player_level: i32,
}

/// Event sent when save/load fails
#[derive(Event, Debug, Clone)]
pub struct SaveLoadFailed {
    pub error_message: String,
}

/// Plugin that handles save/load functionality in Bevy
pub struct SaveLoadPlugin;

impl Plugin for SaveLoadPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SaveGameEvent>()
            .add_event::<LoadGameEvent>()
            .add_event::<GameLoaded>()
            .add_event::<GameSaved>()
            .add_event::<SaveLoadFailed>()
            .add_systems(Startup, try_auto_load)
            .add_systems(Update, (handle_save_event, handle_load_event));
    }
}

/// System that attempts to auto-load the game on startup if a save exists
fn try_auto_load(
    mut player: ResMut<Player>,
    mut storage: ResMut<Storage>,
    mut dungeon: ResMut<DungeonResource>,
    mut game_loaded_events: EventWriter<GameLoaded>,
    mut failed_events: EventWriter<SaveLoadFailed>,
) {
    // Check if a save file exists
    if !save_exists(None) {
        info!("No save file found, starting new game");
        return;
    }

    info!("Save file found, attempting to load...");

    // Try to load the game
    match load_game(None) {
        Ok((loaded_player, loaded_storage, loaded_dungeon)) => {
            // Update resources
            *player = loaded_player;
            *storage = loaded_storage;
            if let Some(d) = loaded_dungeon {
                *dungeon = DungeonResource(d);
            }

            info!("Game loaded successfully!");

            // Send event
            game_loaded_events.send(GameLoaded {
                player_name: player.name.to_string(),
                player_level: player.prog.level,
                player_gold: player.gold,
            });
        }
        Err(e) => {
            error!("Failed to load save file: {}", e);
            failed_events.send(SaveLoadFailed {
                error_message: format!("Failed to load save: {}", e),
            });
        }
    }
}

/// System that handles SaveGameEvent
fn handle_save_event(
    mut save_events: EventReader<SaveGameEvent>,
    player: Res<Player>,
    storage: Res<Storage>,
    dungeon: Res<DungeonResource>,
    mut game_saved_events: EventWriter<GameSaved>,
    mut failed_events: EventWriter<SaveLoadFailed>,
) {
    for _ in save_events.read() {
        info!("Saving game...");

        let dungeon_ref = if dungeon.is_generated {
            Some(&dungeon.0)
        } else {
            None
        };

        match save_game(&player, &storage, dungeon_ref, None) {
            Ok(_) => {
                info!("Game saved successfully!");
                game_saved_events.send(GameSaved {
                    player_name: player.name.to_string(),
                    player_level: player.prog.level,
                });
            }
            Err(e) => {
                error!("Failed to save game: {}", e);
                failed_events.send(SaveLoadFailed {
                    error_message: format!("Failed to save game: {}", e),
                });
            }
        }
    }
}

/// System that handles LoadGameEvent
fn handle_load_event(
    mut load_events: EventReader<LoadGameEvent>,
    mut player: ResMut<Player>,
    mut storage: ResMut<Storage>,
    mut dungeon: ResMut<DungeonResource>,
    mut game_loaded_events: EventWriter<GameLoaded>,
    mut failed_events: EventWriter<SaveLoadFailed>,
) {
    for _ in load_events.read() {
        info!("Loading game...");

        match load_game(None) {
            Ok((loaded_player, loaded_storage, loaded_dungeon)) => {
                // Update resources
                *player = loaded_player;
                *storage = loaded_storage;
                if let Some(d) = loaded_dungeon {
                    *dungeon = DungeonResource(d);
                }

                info!("Game loaded successfully!");

                // Send event
                game_loaded_events.send(GameLoaded {
                    player_name: player.name.to_string(),
                    player_level: player.prog.level,
                    player_gold: player.gold,
                });
            }
            Err(e) => {
                error!("Failed to load game: {}", e);
                failed_events.send(SaveLoadFailed {
                    error_message: format!("Failed to load: {}", e),
                });
            }
        }
    }
}
