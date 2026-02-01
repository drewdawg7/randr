use bevy::prelude::*;

use super::actions::{GameAction, HeldDirection, NavigationDirection};

const REPEAT_INITIAL_DELAY: f32 = 0.3;
const REPEAT_INTERVAL: f32 = 0.1;

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

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<GameAction>()
            .add_message::<NavigationDirection>()
            .init_resource::<NavigationRepeatState>()
            .init_resource::<HeldDirection>()
            .add_systems(PreUpdate, handle_keyboard_input);
    }
}

fn key_to_direction(key: KeyCode) -> Option<NavigationDirection> {
    match key {
        KeyCode::ArrowUp => Some(NavigationDirection::Up),
        KeyCode::ArrowDown => Some(NavigationDirection::Down),
        KeyCode::ArrowLeft => Some(NavigationDirection::Left),
        KeyCode::ArrowRight => Some(NavigationDirection::Right),
        _ => None,
    }
}

fn direction_to_key(dir: NavigationDirection) -> KeyCode {
    match dir {
        NavigationDirection::Up => KeyCode::ArrowUp,
        NavigationDirection::Down => KeyCode::ArrowDown,
        NavigationDirection::Left => KeyCode::ArrowLeft,
        NavigationDirection::Right => KeyCode::ArrowRight,
    }
}

fn handle_keyboard_input(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut repeat: ResMut<NavigationRepeatState>,
    mut held: ResMut<HeldDirection>,
    mut action_writer: MessageWriter<GameAction>,
) {
    let mut new_press = None;
    for key in keyboard.get_just_pressed() {
        if let Some(dir) = key_to_direction(*key) {
            new_press = Some(dir);
        }
    }

    if let Some(dir) = new_press {
        action_writer.write(GameAction::Navigate(dir));
        repeat.direction = Some(dir);
        repeat.timer = Timer::from_seconds(REPEAT_INITIAL_DELAY, TimerMode::Once);
        repeat.repeating = false;
    } else if let Some(dir) = repeat.direction {
        if keyboard.pressed(direction_to_key(dir)) {
            repeat.timer.tick(time.delta());
            if repeat.timer.just_finished() && !repeat.repeating {
                repeat.repeating = true;
                repeat.timer = Timer::from_seconds(REPEAT_INTERVAL, TimerMode::Repeating);
                action_writer.write(GameAction::Navigate(dir));
            } else if repeat.repeating {
                for _ in 0..repeat.timer.times_finished_this_tick() {
                    action_writer.write(GameAction::Navigate(dir));
                }
            }
        } else {
            repeat.direction = None;
            repeat.repeating = false;
        }
    }

    if keyboard.just_pressed(KeyCode::Enter) {
        action_writer.write(GameAction::Select);
    }

    if keyboard.just_pressed(KeyCode::Backspace) {
        action_writer.write(GameAction::Back);
    }

    if keyboard.just_pressed(KeyCode::Tab) {
        if keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight) {
            action_writer.write(GameAction::PrevTab);
        } else {
            action_writer.write(GameAction::NextTab);
        }
    }

    if keyboard.just_pressed(KeyCode::Space) {
        action_writer.write(GameAction::Mine);
    }

    if keyboard.just_pressed(KeyCode::KeyI) {
        action_writer.write(GameAction::OpenInventory);
    }
    if keyboard.just_pressed(KeyCode::KeyP) {
        action_writer.write(GameAction::OpenProfile);
    }
    if keyboard.just_pressed(KeyCode::Slash)
        && (keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight))
    {
        action_writer.write(GameAction::OpenKeybinds);
    }

    if keyboard.just_pressed(KeyCode::Escape) {
        action_writer.write(GameAction::CloseModal);
    }

    if keyboard.just_pressed(KeyCode::KeyB) {
        action_writer.write(GameAction::OpenCompendium);
    }

    if keyboard.just_pressed(KeyCode::KeyK) {
        action_writer.write(GameAction::OpenSkills);
    }

    held.0 = repeat.direction;
}

pub fn clear_game_action_events(mut events: ResMut<Messages<GameAction>>) {
    events.clear();
}
