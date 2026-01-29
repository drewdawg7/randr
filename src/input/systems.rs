use bevy::prelude::*;

use super::actions::{GameAction, HeldDirection, NavigationDirection};

/// Initial delay before key repeat starts (seconds).
const REPEAT_INITIAL_DELAY: f32 = 0.3;
/// Interval between repeated navigation events while held (seconds).
const REPEAT_INTERVAL: f32 = 0.1;

/// Tracks key-repeat state for arrow key navigation.
#[derive(Resource)]
struct NavigationRepeatState {
    direction: Option<NavigationDirection>,
    timer: Timer,
    repeating: bool,
}

impl Default for NavigationRepeatState {
    fn default() -> Self {
        Self {
            direction: None,
            timer: Timer::from_seconds(REPEAT_INITIAL_DELAY, TimerMode::Once),
            repeating: false,
        }
    }
}

/// Plugin that handles keyboard input and emits game action events.
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameAction>()
            .add_event::<NavigationDirection>()
            .init_resource::<NavigationRepeatState>()
            .init_resource::<HeldDirection>()
            .add_systems(PreUpdate, handle_keyboard_input);
    }
}

/// Map a key code to a navigation direction.
fn key_to_direction(key: KeyCode) -> Option<NavigationDirection> {
    match key {
        KeyCode::ArrowUp => Some(NavigationDirection::Up),
        KeyCode::ArrowDown => Some(NavigationDirection::Down),
        KeyCode::ArrowLeft => Some(NavigationDirection::Left),
        KeyCode::ArrowRight => Some(NavigationDirection::Right),
        _ => None,
    }
}

/// Map a navigation direction back to a key code.
fn direction_to_key(dir: NavigationDirection) -> KeyCode {
    match dir {
        NavigationDirection::Up => KeyCode::ArrowUp,
        NavigationDirection::Down => KeyCode::ArrowDown,
        NavigationDirection::Left => KeyCode::ArrowLeft,
        NavigationDirection::Right => KeyCode::ArrowRight,
    }
}

/// System that reads keyboard input and emits GameAction events.
/// Keyboard-only - no mouse input per the rewrite spec.
fn handle_keyboard_input(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut repeat: ResMut<NavigationRepeatState>,
    mut held: ResMut<HeldDirection>,
    mut action_writer: EventWriter<GameAction>,
) {
    // Navigation - Arrow keys with key repeat
    let mut new_press = None;
    for key in keyboard.get_just_pressed() {
        if let Some(dir) = key_to_direction(*key) {
            new_press = Some(dir);
        }
    }

    if let Some(dir) = new_press {
        // Fresh key press: emit immediately and start initial delay
        action_writer.send(GameAction::Navigate(dir));
        repeat.direction = Some(dir);
        repeat.timer = Timer::from_seconds(REPEAT_INITIAL_DELAY, TimerMode::Once);
        repeat.repeating = false;
    } else if let Some(dir) = repeat.direction {
        // Check if the held key is still pressed
        if keyboard.pressed(direction_to_key(dir)) {
            repeat.timer.tick(time.delta());
            if repeat.timer.just_finished() && !repeat.repeating {
                // Initial delay elapsed, emit first repeat and switch to repeat interval
                repeat.repeating = true;
                repeat.timer = Timer::from_seconds(REPEAT_INTERVAL, TimerMode::Repeating);
                action_writer.send(GameAction::Navigate(dir));
            } else if repeat.repeating {
                // Emit repeated events
                for _ in 0..repeat.timer.times_finished_this_tick() {
                    action_writer.send(GameAction::Navigate(dir));
                }
            }
        } else {
            // Key released, reset state
            repeat.direction = None;
            repeat.repeating = false;
        }
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

    // Monster compendium - b
    if keyboard.just_pressed(KeyCode::KeyB) {
        action_writer.send(GameAction::OpenCompendium);
    }

    // Sync HeldDirection with repeat state
    held.0 = repeat.direction;
}

/// Clear all pending GameAction events. Use in OnExit to prevent event bleed.
pub fn clear_game_action_events(mut events: ResMut<Events<GameAction>>) {
    events.clear();
}
