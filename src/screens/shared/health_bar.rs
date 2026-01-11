use bevy::prelude::*;

use crate::assets::SpriteSheet;

/// Component marker for a health bar container.
#[derive(Component)]
pub struct HealthBar;

/// Component marker for a sprite-based health bar.
#[derive(Component)]
pub struct SpriteHealthBar;

/// Bundle for a header label (e.g., "PLAYER", "ENEMY").
#[derive(Bundle)]
pub struct HeaderLabelBundle {
    pub text: Text,
    pub font: TextFont,
    pub color: TextColor,
    pub node: Node,
}

impl HeaderLabelBundle {
    pub fn new(label: &str, color: Color) -> Self {
        Self {
            text: Text::new(label),
            font: TextFont {
                font_size: 24.0,
                ..default()
            },
            color: TextColor(color),
            node: Node {
                margin: UiRect::bottom(Val::Px(10.0)),
                ..default()
            },
        }
    }
}

/// Bundle for the health bar container.
#[derive(Bundle)]
pub struct HealthBarBundle {
    pub bar: HealthBar,
    pub node: Node,
}

impl HealthBarBundle {
    pub fn new(width: f32) -> Self {
        Self {
            bar: HealthBar,
            node: Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(5.0),
                width: Val::Px(width),
                ..default()
            },
        }
    }
}

/// Bundle for the name label inside a health bar.
#[derive(Bundle)]
pub struct HealthBarNameBundle {
    pub text: Text,
    pub font: TextFont,
    pub color: TextColor,
}

impl HealthBarNameBundle {
    pub fn new(name: &str) -> Self {
        Self {
            text: Text::new(name),
            font: TextFont {
                font_size: 18.0,
                ..default()
            },
            color: TextColor(Color::WHITE),
        }
    }
}

/// Bundle for the health bar text display.
#[derive(Bundle)]
pub struct HealthBarTextBundle {
    pub marker: HealthBarText,
    pub text: Text,
    pub font: TextFont,
    pub color: TextColor,
}

impl HealthBarTextBundle {
    pub fn new(health: i32, max_health: i32) -> Self {
        Self {
            marker: HealthBarText,
            text: Text::new(format!("{}/{}", health, max_health)),
            font: TextFont {
                font_size: 14.0,
                ..default()
            },
            color: TextColor(Color::srgb(0.8, 0.8, 0.8)),
        }
    }
}

/// Component marker for the health text display.
#[derive(Component)]
pub struct HealthBarText;

/// Get the sprite slice name for a given health percentage.
///
/// Returns the appropriate slice from the health bar sprite sequence:
/// Full (100%) -> Empty (0%): 2933 -> 2934 -> ... -> 2937 -> 2940 -> ... -> 2944 -> 2938
fn health_bar_slice(percent: f32) -> &'static str {
    const SLICES: [&str; 11] = [
        "Slice_2938", // 0% - empty
        "Slice_2944", // ~9%
        "Slice_2943", // ~18%
        "Slice_2942", // ~27%
        "Slice_2941", // ~36%
        "Slice_2940", // ~45%
        "Slice_2937", // ~55%
        "Slice_2936", // ~64%
        "Slice_2935", // ~73%
        "Slice_2934", // ~82%
        "Slice_2933", // 91-100% - full
    ];
    let index = ((percent / 100.0) * 10.0).round() as usize;
    SLICES[index.min(10)]
}

/// Bundle for a sprite-based health bar.
#[derive(Bundle)]
pub struct SpriteHealthBarBundle {
    pub marker: SpriteHealthBar,
    pub node: Node,
    pub image: ImageNode,
}

impl SpriteHealthBarBundle {
    /// Create a new sprite health bar.
    ///
    /// Returns None if the sprite sheet doesn't contain the required slices.
    pub fn new(health: i32, max_health: i32, sheet: &SpriteSheet) -> Option<Self> {
        let percent = if max_health > 0 {
            (health as f32 / max_health as f32 * 100.0).clamp(0.0, 100.0)
        } else {
            0.0
        };
        let slice_name = health_bar_slice(percent);
        let image = sheet.image_node(slice_name)?;

        Some(Self {
            marker: SpriteHealthBar,
            node: Node {
                width: Val::Px(200.0),
                height: Val::Px(100.0),
                ..default()
            },
            image,
        })
    }
}

/// Update a sprite-based health bar's image and text based on new values.
pub fn update_health_bar(
    bar_entity: Entity,
    current: i32,
    max: i32,
    children: &Query<&Children>,
    sprite_query: &mut Query<&mut ImageNode, With<SpriteHealthBar>>,
    text_query: &mut Query<&mut Text, With<HealthBarText>>,
    sheet: &SpriteSheet,
) {
    let percent = if max > 0 {
        (current as f32 / max as f32 * 100.0).clamp(0.0, 100.0)
    } else {
        0.0
    };
    let slice_name = health_bar_slice(percent);

    // Find sprite and text components in children
    if let Ok(bar_children) = children.get(bar_entity) {
        for child in bar_children.iter() {
            // Update sprite
            if let Ok(mut image_node) = sprite_query.get_mut(*child) {
                if let Some(index) = sheet.get(slice_name) {
                    if let Some(atlas) = &mut image_node.texture_atlas {
                        atlas.index = index;
                    }
                }
            }
            // Update text
            if let Ok(mut text) = text_query.get_mut(*child) {
                **text = format!("{}/{}", current, max);
            }
        }
    }
}
