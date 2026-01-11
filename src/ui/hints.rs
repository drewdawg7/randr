//! Navigation hint helpers for UI screens.

use bevy::prelude::*;

use super::theme::colors;

/// Font sizes for hint text.
const TOWN_FONT_SIZE: f32 = 14.0;
const MODAL_FONT_SIZE: f32 = 16.0;

/// Spawn a navigation hint for town screens.
///
/// Uses smaller text (14px) with darker gray color.
/// Uses `margin: UiRect::top(Val::Auto)` to push the hint to the bottom
/// of a flex column container.
pub fn spawn_navigation_hint(parent: &mut ChildBuilder, hint: &str) {
    parent.spawn((
        Text::new(hint),
        TextFont {
            font_size: TOWN_FONT_SIZE,
            ..default()
        },
        TextColor(colors::HINT_TOWN),
        Node {
            margin: UiRect::top(Val::Auto),
            ..default()
        },
    ));
}

/// Spawn a navigation hint for modal screens.
///
/// Uses larger text (16px) with lighter gray color.
/// Does not include auto-margin - caller controls positioning.
pub fn spawn_modal_hint(parent: &mut ChildBuilder, hint: &str) {
    parent.spawn((
        Text::new(hint),
        TextFont {
            font_size: MODAL_FONT_SIZE,
            ..default()
        },
        TextColor(colors::HINT_MODAL),
    ));
}
