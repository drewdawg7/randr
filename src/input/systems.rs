use bevy::prelude::*;

use super::actions::{GameAction, NavigationDirection};
use crate::states::AppState;

/// Plugin that handles keyboard input and emits game action events.
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameAction>()
            .add_event::<NavigationDirection>()
            .add_systems(PreUpdate, handle_keyboard_input)
            .add_systems(Update, handle_global_keybinds_action);
    }
}

/// System that reads keyboard input and emits GameAction events.
/// Keyboard-only - no mouse input per the rewrite spec.
fn handle_keyboard_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut action_writer: EventWriter<GameAction>,
) {
    // Navigation - Arrow keys
    if keyboard.just_pressed(KeyCode::ArrowUp) {
        action_writer.send(GameAction::Navigate(NavigationDirection::Up));
    }
    if keyboard.just_pressed(KeyCode::ArrowDown) {
        action_writer.send(GameAction::Navigate(NavigationDirection::Down));
    }
    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        action_writer.send(GameAction::Navigate(NavigationDirection::Left));
    }
    if keyboard.just_pressed(KeyCode::ArrowRight) {
        action_writer.send(GameAction::Navigate(NavigationDirection::Right));
    }

    // Select/confirm - Enter
    if keyboard.just_pressed(KeyCode::Enter) {
        action_writer.send(GameAction::Select);
    }

    // Back/cancel - Backspace
    if keyboard.just_pressed(KeyCode::Backspace) {
        action_writer.send(GameAction::Back);
    }

    // Tab switching
    if keyboard.just_pressed(KeyCode::Tab) {
        if keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight) {
            action_writer.send(GameAction::PrevTab);
        } else {
            action_writer.send(GameAction::NextTab);
        }
    }

    // Mining action - Space
    if keyboard.just_pressed(KeyCode::Space) {
        action_writer.send(GameAction::Mine);
    }

    // Modal toggles
    if keyboard.just_pressed(KeyCode::KeyI) {
        action_writer.send(GameAction::OpenInventory);
    }
    if keyboard.just_pressed(KeyCode::KeyP) {
        action_writer.send(GameAction::OpenProfile);
    }
    if keyboard.just_pressed(KeyCode::Slash)
        && (keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight))
    {
        // ? key (Shift + /)
        action_writer.send(GameAction::OpenKeybinds);
    }

    // Close modal - Escape
    if keyboard.just_pressed(KeyCode::Escape) {
        action_writer.send(GameAction::CloseModal);
    }

    // Book popup - b
    if keyboard.just_pressed(KeyCode::KeyB) {
        action_writer.send(GameAction::OpenBook);
    }
}

/// Clear all pending GameAction events. Use in OnExit to prevent event bleed.
pub fn clear_game_action_events(mut events: ResMut<Events<GameAction>>) {
    events.clear();
}

/// Global system to handle OpenKeybinds action from any state.
/// This allows the keybinds modal to be opened from anywhere in the game.
fn handle_global_keybinds_action(
    mut action_reader: EventReader<GameAction>,
    mut next_state: ResMut<NextState<AppState>>,
    current_state: Res<State<AppState>>,
) {
    for action in action_reader.read() {
        if *action == GameAction::OpenKeybinds && **current_state != AppState::Keybinds {
            next_state.set(AppState::Keybinds);
        }
    }
}
