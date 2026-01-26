use bevy::prelude::*;

use crate::assets::GameFonts;

/// Configuration for spawning outlined quantity text.
pub struct OutlinedQuantityConfig {
    /// Font size (default: 14.0)
    pub font_size: f32,
    /// Main text color (default: WHITE)
    pub text_color: Color,
    /// Outline color (default: BLACK)
    pub outline_color: Color,
    /// Position from right edge (default: 2.0)
    pub right: f32,
    /// Position from bottom edge (default: 0.0)
    pub bottom: f32,
}

impl Default for OutlinedQuantityConfig {
    fn default() -> Self {
        Self {
            font_size: 14.0,
            text_color: Color::WHITE,
            outline_color: Color::BLACK,
            right: 2.0,
            bottom: 0.0,
        }
    }
}

/// Spawn quantity text with a black outline effect at the bottom-right corner.
/// Creates shadow text entities at 8 offsets around the main text for a thick outline.
///
/// The `marker` parameter allows callers to attach a marker component to the container
/// entity for later querying (e.g., for despawning on refresh).
pub fn spawn_outlined_quantity_text<M: Bundle>(
    parent: &mut ChildBuilder,
    game_fonts: &GameFonts,
    quantity: u32,
    config: OutlinedQuantityConfig,
    marker: M,
) {
    let text = quantity.to_string();
    let font = game_fonts.pixel_font(config.font_size);

    // Shadow offsets - 8 directions for thicker outline
    const OFFSETS: [(f32, f32); 8] = [
        (-1.0, -1.0),
        (0.0, -1.0),
        (1.0, -1.0),
        (-1.0, 0.0),
        (1.0, 0.0),
        (-1.0, 1.0),
        (0.0, 1.0),
        (1.0, 1.0),
    ];

    parent
        .spawn((
            marker,
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(config.right),
                bottom: Val::Px(config.bottom),
                ..default()
            },
        ))
        .with_children(|text_container| {
            // Shadow layers (outline)
            for (x, y) in OFFSETS {
                text_container.spawn((
                    Text::new(&text),
                    font.clone(),
                    TextColor(config.outline_color),
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(x),
                        top: Val::Px(y),
                        ..default()
                    },
                ));
            }

            // Main text on top
            text_container.spawn((
                Text::new(&text),
                font,
                TextColor(config.text_color),
            ));
        });
}

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
