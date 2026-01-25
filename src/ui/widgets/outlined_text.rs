use bevy::prelude::*;

use crate::assets::GameFonts;

/// Plugin for outlined text widget.
pub struct OutlinedTextPlugin;

impl Plugin for OutlinedTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_add_outlined_text);
    }
}

/// Widget that displays text with an outline effect.
///
/// Uses layered shadow text nodes behind the main text to create an outline.
/// Spawns a relative container with 4 shadow copies offset in cardinal directions,
/// plus the main text on top.
#[derive(Component)]
pub struct OutlinedText {
    /// The text to display.
    pub text: String,
    /// Font size.
    pub font_size: f32,
    /// Main text color.
    pub text_color: Color,
    /// Outline/shadow color.
    pub outline_color: Color,
    /// Pixel offset for outline shadows.
    pub outline_offset: f32,
}

impl OutlinedText {
    /// Create a new outlined text with the given content.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            font_size: 16.0,
            text_color: Color::WHITE,
            outline_color: Color::BLACK,
            outline_offset: 1.0,
        }
    }

    /// Set the font size.
    pub fn with_font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    /// Set the main text color.
    pub fn with_color(mut self, color: Color) -> Self {
        self.text_color = color;
        self
    }

    /// Set the outline color.
    pub fn with_outline(mut self, color: Color) -> Self {
        self.outline_color = color;
        self
    }

    /// Set the outline offset in pixels.
    pub fn with_outline_offset(mut self, offset: f32) -> Self {
        self.outline_offset = offset;
        self
    }
}

fn on_add_outlined_text(
    trigger: Trigger<OnAdd, OutlinedText>,
    mut commands: Commands,
    query: Query<&OutlinedText>,
    game_fonts: Res<GameFonts>,
) {
    let entity = trigger.entity();
    let Ok(outlined) = query.get(entity) else {
        return;
    };

    // Capture values before removing component
    let text = outlined.text.clone();
    let font_size = outlined.font_size;
    let text_color = outlined.text_color;
    let outline_color = outlined.outline_color;
    let offset = outlined.outline_offset;

    let font = game_fonts.pixel_font(font_size);

    commands
        .entity(entity)
        .remove::<OutlinedText>()
        .insert(Node {
            position_type: PositionType::Relative,
            ..default()
        })
        .with_children(|parent| {
            // Shadow offsets for 4 cardinal directions
            let offsets = [
                (offset, 0.0),   // right
                (-offset, 0.0),  // left
                (0.0, offset),   // down
                (0.0, -offset),  // up
            ];

            // Spawn shadow text nodes
            for (x, y) in offsets {
                parent.spawn((
                    Text::new(&text),
                    font.clone(),
                    TextColor(outline_color),
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(x),
                        top: Val::Px(y),
                        ..default()
                    },
                ));
            }

            // Spawn main text on top
            parent.spawn((
                Text::new(&text),
                font,
                TextColor(text_color),
                Node {
                    position_type: PositionType::Relative,
                    ..default()
                },
            ));
        });
}
