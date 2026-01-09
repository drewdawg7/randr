//! Spell Test Modal - "God Mode" for testing word combinations.
//! Allows entering any words to see the resulting spell effect.

use bevy::prelude::*;

use crate::input::GameAction;
use crate::magic::spell::compute_spell;
use crate::magic::word::WordId;

use super::modal::{spawn_modal_overlay, ActiveModal, ModalType};

/// Plugin that manages the spell test modal.
pub struct SpellTestModalPlugin;

impl Plugin for SpellTestModalPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpellTestState>().add_systems(
            Update,
            (
                handle_spell_test_modal_toggle,
                handle_spell_test_input,
                update_spell_test_display,
            ),
        );
    }
}

/// Component marker for the spell test modal UI.
#[derive(Component)]
pub struct SpellTestModalRoot;

/// Components for identifying text elements in the modal.
#[derive(Component)]
struct InputText;

#[derive(Component)]
struct ResultText;

#[derive(Component)]
struct AvailableWordsText;

#[derive(Component)]
struct PageSelectorText;

/// Resource for tracking spell test modal state.
#[derive(Resource, Default)]
struct SpellTestState {
    input: String,
    result: Option<SpellTestResult>,
    selected_page: usize, // 0, 1, or 2
}

impl SpellTestState {
    fn reset(&mut self) {
        self.input.clear();
        self.result = None;
        self.selected_page = 0;
    }

    fn test_spell(&mut self) {
        if self.input.trim().is_empty() {
            return;
        }

        let words: Vec<&str> = self.input.split_whitespace().collect();
        let mut parsed = Vec::new();
        let mut unknown = Vec::new();

        for word in words {
            if let Some(word_id) = WordId::from_str(word) {
                parsed.push(word_id);
            } else {
                unknown.push(word.to_string());
            }
        }

        if !unknown.is_empty() {
            self.result = Some(SpellTestResult::ParseError { unknown });
            return;
        }

        if parsed.is_empty() {
            return;
        }

        // Compute the spell
        let spell = compute_spell(&parsed);

        self.result = Some(match spell {
            crate::magic::spell::ComputedSpell::Active {
                name, description, ..
            } => SpellTestResult::Success {
                spell_name: name,
                description,
                is_backfire: false,
            },
            crate::magic::spell::ComputedSpell::Passive {
                name, description, ..
            } => SpellTestResult::Success {
                spell_name: format!("{} (Passive)", name),
                description,
                is_backfire: false,
            },
            crate::magic::spell::ComputedSpell::Hybrid {
                name, description, ..
            } => SpellTestResult::Success {
                spell_name: format!("{} (Hybrid)", name),
                description,
                is_backfire: false,
            },
            crate::magic::spell::ComputedSpell::Backfire { reason, effect } => {
                SpellTestResult::Success {
                    spell_name: "Unstable Magic".to_string(),
                    description: format!("{}: {}", reason, effect.describe()),
                    is_backfire: true,
                }
            }
            crate::magic::spell::ComputedSpell::Fizzle { reason } => {
                SpellTestResult::Fizzle { reason }
            }
        });
    }
}

#[derive(Clone)]
enum SpellTestResult {
    Success {
        spell_name: String,
        description: String,
        is_backfire: bool,
    },
    Fizzle {
        reason: String,
    },
    ParseError {
        unknown: Vec<String>,
    },
}

/// System to handle opening/closing the spell test modal with 'M' key.
fn handle_spell_test_modal_toggle(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut active_modal: ResMut<ActiveModal>,
    mut state: ResMut<SpellTestState>,
    existing_modal: Query<Entity, With<SpellTestModalRoot>>,
) {
    if keyboard.just_pressed(KeyCode::KeyM) {
        // Toggle: close if open, open if closed
        if let Ok(entity) = existing_modal.get_single() {
            commands.entity(entity).despawn_recursive();
            active_modal.modal = None;
        } else {
            state.reset();
            spawn_spell_test_modal(&mut commands);
            active_modal.modal = Some(ModalType::SpellTest);
        }
    }

    // Also handle CloseModal action
    if active_modal.modal == Some(ModalType::SpellTest) {
        if keyboard.just_pressed(KeyCode::Escape) {
            if let Ok(entity) = existing_modal.get_single() {
                commands.entity(entity).despawn_recursive();
                active_modal.modal = None;
            }
        }
    }
}

/// System to handle input when spell test modal is open.
fn handle_spell_test_input(
    mut action_reader: EventReader<GameAction>,
    active_modal: Res<ActiveModal>,
    mut state: ResMut<SpellTestState>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if active_modal.modal != Some(ModalType::SpellTest) {
        return;
    }

    // Handle game actions
    for action in action_reader.read() {
        match action {
            GameAction::Navigate(dir) => {
                use crate::input::NavigationDirection;
                match dir {
                    NavigationDirection::Left => {
                        if state.selected_page > 0 {
                            state.selected_page -= 1;
                        }
                    }
                    NavigationDirection::Right => {
                        if state.selected_page < 2 {
                            state.selected_page += 1;
                        }
                    }
                    _ => {}
                }
            }
            GameAction::Select => {
                state.test_spell();
            }
            _ => {}
        }
    }

    // Handle text input
    if keyboard.just_pressed(KeyCode::Backspace) {
        state.input.pop();
        state.result = None;
    }

    if keyboard.just_pressed(KeyCode::Space) {
        if state.input.len() < 50 {
            state.input.push(' ');
        }
    }

    // Handle alphanumeric characters
    for key in keyboard.get_just_pressed() {
        if let Some(c) = key_to_char(*key, &keyboard) {
            if state.input.len() < 50 {
                state.input.push(c);
            }
        }
    }
}

/// Convert KeyCode to character, handling shift for uppercase.
fn key_to_char(key: KeyCode, keyboard: &ButtonInput<KeyCode>) -> Option<char> {
    let shift = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);

    match key {
        KeyCode::KeyA => Some(if shift { 'A' } else { 'a' }),
        KeyCode::KeyB => Some(if shift { 'B' } else { 'b' }),
        KeyCode::KeyC => Some(if shift { 'C' } else { 'c' }),
        KeyCode::KeyD => Some(if shift { 'D' } else { 'd' }),
        KeyCode::KeyE => Some(if shift { 'E' } else { 'e' }),
        KeyCode::KeyF => Some(if shift { 'F' } else { 'f' }),
        KeyCode::KeyG => Some(if shift { 'G' } else { 'g' }),
        KeyCode::KeyH => Some(if shift { 'H' } else { 'h' }),
        // Skip I - used for inventory
        KeyCode::KeyJ => Some(if shift { 'J' } else { 'j' }),
        KeyCode::KeyK => Some(if shift { 'K' } else { 'k' }),
        KeyCode::KeyL => Some(if shift { 'L' } else { 'l' }),
        // Skip M - used to toggle modal
        KeyCode::KeyN => Some(if shift { 'N' } else { 'n' }),
        KeyCode::KeyO => Some(if shift { 'O' } else { 'o' }),
        // Skip P - used for profile
        KeyCode::KeyQ => Some(if shift { 'Q' } else { 'q' }),
        KeyCode::KeyR => Some(if shift { 'R' } else { 'r' }),
        KeyCode::KeyS => Some(if shift { 'S' } else { 's' }),
        KeyCode::KeyT => Some(if shift { 'T' } else { 't' }),
        KeyCode::KeyU => Some(if shift { 'U' } else { 'u' }),
        KeyCode::KeyV => Some(if shift { 'V' } else { 'v' }),
        KeyCode::KeyW => Some(if shift { 'W' } else { 'w' }),
        KeyCode::KeyX => Some(if shift { 'X' } else { 'x' }),
        KeyCode::KeyY => Some(if shift { 'Y' } else { 'y' }),
        KeyCode::KeyZ => Some(if shift { 'Z' } else { 'z' }),
        KeyCode::Digit0 => Some('0'),
        KeyCode::Digit1 => Some('1'),
        KeyCode::Digit2 => Some('2'),
        KeyCode::Digit3 => Some('3'),
        KeyCode::Digit4 => Some('4'),
        KeyCode::Digit5 => Some('5'),
        KeyCode::Digit6 => Some('6'),
        KeyCode::Digit7 => Some('7'),
        KeyCode::Digit8 => Some('8'),
        KeyCode::Digit9 => Some('9'),
        _ => None,
    }
}

/// Spawn the spell test modal UI.
fn spawn_spell_test_modal(commands: &mut Commands) {
    let overlay_id = spawn_modal_overlay(commands);

    commands.entity(overlay_id).with_children(|overlay| {
        overlay
            .spawn((
                SpellTestModalRoot,
                Node {
                    width: Val::Px(700.0),
                    height: Val::Px(550.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(30.0)),
                    border: UiRect::all(Val::Px(3.0)),
                    row_gap: Val::Px(15.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.176, 0.157, 0.137)),
                BorderColor(Color::srgb(0.38, 0.29, 0.22)),
            ))
            .with_children(|modal| {
                // Title
                modal.spawn((
                    Text::new("✦ Spell Tester (God Mode) ✦"),
                    TextFont {
                        font_size: 32.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.91, 0.82, 0.61)),
                    Node {
                        margin: UiRect::bottom(Val::Px(10.0)),
                        align_self: AlignSelf::Center,
                        ..default()
                    },
                ));

                // Instructions
                modal.spawn((
                    Text::new("Enter words separated by spaces, then press Enter"),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.71, 0.62, 0.48)),
                    Node {
                        margin: UiRect::bottom(Val::Px(10.0)),
                        ..default()
                    },
                ));

                // Input field
                modal
                    .spawn(Node {
                        width: Val::Percent(100.0),
                        padding: UiRect::all(Val::Px(10.0)),
                        margin: UiRect::bottom(Val::Px(15.0)),
                        ..default()
                    })
                    .with_children(|input_container| {
                        input_container.spawn((
                            InputText,
                            Text::new(""),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });

                // Result area
                modal
                    .spawn(Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(250.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(15.0)),
                        overflow: Overflow::clip_y(),
                        row_gap: Val::Px(10.0),
                        ..default()
                    })
                    .with_children(|result_area| {
                        // Result text
                        result_area.spawn((
                            ResultText,
                            Text::new(""),
                            TextFont {
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.91, 0.91, 0.78)),
                        ));

                        // Available words
                        result_area.spawn((
                            AvailableWordsText,
                            Text::new(format_available_words()),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.71, 0.62, 0.48)),
                        ));

                        // Page selector
                        result_area.spawn((
                            PageSelectorText,
                            Text::new(""),
                            TextFont {
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.71, 0.62, 0.48)),
                        ));
                    });

                // Footer instructions
                modal.spawn((
                    Text::new("[Enter] Test  [M] Close  [←→] Page  [Esc] Close"),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.4, 0.33, 0.27)),
                    Node {
                        margin: UiRect::top(Val::Px(10.0)),
                        align_self: AlignSelf::Center,
                        ..default()
                    },
                ));
            });
    });
}

/// System to update the spell test modal display when state changes.
fn update_spell_test_display(
    active_modal: Res<ActiveModal>,
    state: Res<SpellTestState>,
    mut input_query: Query<
        &mut Text,
        (
            With<InputText>,
            Without<ResultText>,
            Without<AvailableWordsText>,
            Without<PageSelectorText>,
        ),
    >,
    mut result_query: Query<
        &mut Text,
        (
            With<ResultText>,
            Without<InputText>,
            Without<AvailableWordsText>,
            Without<PageSelectorText>,
        ),
    >,
    mut available_query: Query<
        &mut Text,
        (
            With<AvailableWordsText>,
            Without<InputText>,
            Without<ResultText>,
            Without<PageSelectorText>,
        ),
    >,
    mut page_query: Query<
        &mut Text,
        (
            With<PageSelectorText>,
            Without<InputText>,
            Without<ResultText>,
            Without<AvailableWordsText>,
        ),
    >,
) {
    if active_modal.modal != Some(ModalType::SpellTest) {
        return;
    }

    if !state.is_changed() {
        return;
    }

    // Update input text
    if let Ok(mut text) = input_query.get_single_mut() {
        **text = if state.input.is_empty() {
            "Type words here...".to_string()
        } else {
            format!("{}▌", state.input)
        };
    }

    // Update result text
    if let Ok(mut text) = result_query.get_single_mut() {
        **text = format_result(&state.result);
    }

    // Update available words (hide if result exists)
    if let Ok(mut text) = available_query.get_single_mut() {
        if state.result.is_none() {
            **text = format_available_words();
        } else {
            **text = String::new();
        }
    }

    // Update page selector
    if let Ok(mut text) = page_query.get_single_mut() {
        **text = format_page_selector(&state);
    }
}

fn format_result(result: &Option<SpellTestResult>) -> String {
    match result {
        Some(SpellTestResult::Success {
            spell_name,
            description,
            is_backfire,
        }) => {
            if *is_backfire {
                format!("⚠ BACKFIRE: {}\n\n{}", spell_name, description)
            } else {
                format!("✓ Spell: {}\n\n{}", spell_name, description)
            }
        }
        Some(SpellTestResult::Fizzle { reason }) => {
            format!("✗ Fizzle: {}", reason)
        }
        Some(SpellTestResult::ParseError { unknown }) => {
            format!("✗ Unknown words: {}", unknown.join(", "))
        }
        None => String::new(),
    }
}

fn format_available_words() -> String {
    let all_words: Vec<&str> = WordId::all().iter().map(|w| w.name()).collect();
    format!("Available words:\n{}", all_words.join(", "))
}

fn format_page_selector(state: &SpellTestState) -> String {
    if state.result.is_some()
        && !matches!(&state.result, Some(SpellTestResult::ParseError { .. }))
    {
        let mut page_str = String::from("Inscribe to page: ");
        for i in 0..3 {
            if i == state.selected_page {
                page_str.push_str(&format!("[{}*] ", i + 1));
            } else {
                page_str.push_str(&format!("[{}] ", i + 1));
            }
        }
        page_str
    } else {
        String::new()
    }
}
