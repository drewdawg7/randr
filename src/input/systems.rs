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
}

/// Resource for tracking list/menu selection state.
/// Ported from the original ListSelection pattern.
#[derive(Resource, Default)]
pub struct ListSelection {
    pub index: usize,
    pub count: usize,
}

impl ListSelection {
    pub fn new(count: usize) -> Self {
        Self { index: 0, count }
    }

    pub fn up(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        }
    }

    pub fn down(&mut self) {
        if self.index + 1 < self.count {
            self.index += 1;
        }
    }

    pub fn set_count(&mut self, count: usize) {
        self.count = count;
        if self.index >= count && count > 0 {
            self.index = count - 1;
        }
    }
}

/// Resource for tracking 2D grid selection state.
/// Ported from the original GridSelection pattern.
#[derive(Resource, Default)]
pub struct GridSelection {
    pub row: usize,
    pub col: usize,
    pub rows: usize,
    pub cols: usize,
}

impl GridSelection {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            row: 0,
            col: 0,
            rows,
            cols,
        }
    }

    pub fn up(&mut self) {
        if self.row > 0 {
            self.row -= 1;
        }
    }

    pub fn down(&mut self) {
        if self.row + 1 < self.rows {
            self.row += 1;
        }
    }

    pub fn left(&mut self) {
        if self.col > 0 {
            self.col -= 1;
        }
    }

    pub fn right(&mut self) {
        if self.col + 1 < self.cols {
            self.col += 1;
        }
    }

    /// Get the linear index from row/col position.
    pub fn index(&self) -> usize {
        self.row * self.cols + self.col
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
