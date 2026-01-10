//! Navigation hint helper for UI screens.

use bevy::prelude::*;

/// Spawn a navigation hint at the bottom of the screen.
///
/// Uses `margin: UiRect::top(Val::Auto)` to push the hint to the bottom
/// of a flex column container.
pub fn spawn_navigation_hint(parent: &mut ChildBuilder, hint: &str) {
    parent.spawn((
        Text::new(hint),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        TextColor(Color::srgb(0.5, 0.5, 0.5)),
        Node {
            margin: UiRect::top(Val::Auto),
            ..default()
        },
    ));
}
