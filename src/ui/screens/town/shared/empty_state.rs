//! Empty state message helper for UI screens.

use bevy::prelude::*;

/// Spawn an empty state message (e.g., "No items to display").
///
/// Uses gray text at body size for a subtle appearance.
pub fn spawn_empty_state(parent: &mut ChildBuilder, message: &str) {
    parent.spawn((
        Text::new(message),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::srgb(0.6, 0.6, 0.6)),
    ));
}
