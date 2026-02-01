use bevy::prelude::*;
use bevy::ui::widget::NodeImageMode;

use crate::assets::{GameSprites, HealthBarSlice, SpriteSheet, SpriteSheetKey};
use crate::mob::Health;

/// Component marker for a health bar container.
#[derive(Component)]
pub struct HealthBar;

/// Component marker for a sprite-based health bar.
/// Spawned as a placeholder, then populated by `init_sprite_health_bars`.
#[derive(Component)]
pub struct SpriteHealthBar;

/// Bundle for the health bar container.
#[derive(Bundle)]
pub struct HealthBarBundle {
    pub bar: HealthBar,
    pub node: Node,
}

impl HealthBarBundle {
    pub fn new(align: AlignItems) -> Self {
        Self {
            bar: HealthBar,
            node: Node {
                flex_direction: FlexDirection::Column,
                align_items: align,
                row_gap: Val::Px(5.0),
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


/// Get the sprite slice for a given health percentage.
fn health_bar_slice(percent: f32) -> HealthBarSlice {
    HealthBarSlice::for_percent(percent)
}

/// Bundle for a sprite-based health bar placeholder.
/// The ImageNode is added later by `init_sprite_health_bars`.
#[derive(Bundle)]
pub struct SpriteHealthBarBundle {
    pub marker: SpriteHealthBar,
    pub health: Health,
    pub node: Node,
}

impl SpriteHealthBarBundle {
    pub fn new(current: i32, max: i32, width: f32, height: f32) -> Self {
        Self {
            marker: SpriteHealthBar,
            health: Health { current, max },
            node: Node {
                width: Val::Px(width),
                height: Val::Px(height),
                ..default()
            },
        }
    }
}

/// System to initialize sprite health bars that don't have an ImageNode yet.
pub fn init_sprite_health_bars(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    mut query: Query<(Entity, &Health, &mut Node), (With<SpriteHealthBar>, Without<ImageNode>)>,
) {
    let Some(sheet) = game_sprites.get(SpriteSheetKey::UiAll) else {
        return;
    };

    for (entity, health, mut node) in &mut query {
        let percent = if health.max > 0 {
            (health.current as f32 / health.max as f32 * 100.0).clamp(0.0, 100.0)
        } else {
            0.0
        };
        let slice = HealthBarSlice::for_percent(percent);

        let Some(mut image) = sheet.image_node(slice.as_str()) else {
            continue;
        };
        image.image_mode = NodeImageMode::Sliced(TextureSlicer {
            border: BorderRect {
                min_inset: Vec2::new(3.0, 2.0),
                max_inset: Vec2::new(3.0, 2.0),
            },
            ..default()
        });

        node.justify_content = JustifyContent::Center;
        node.align_items = AlignItems::Center;

        let hp_text = format!("{} / {}", health.current, health.max);
        commands.entity(entity).insert(image).with_child((
            HealthBarText,
            Text::new(hp_text),
            TextFont {
                font_size: 10.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));
    }
}

pub fn update_sprite_health_bar_visuals(
    game_sprites: Res<GameSprites>,
    mut bar_query: Query<
        (&Health, &mut ImageNode, &Children),
        (With<SpriteHealthBar>, Changed<Health>),
    >,
    mut text_query: Query<&mut Text, With<HealthBarText>>,
) {
    let Some(sheet) = game_sprites.get(SpriteSheetKey::UiAll) else {
        return;
    };

    for (health, mut image, children) in &mut bar_query {
        let percent = if health.max > 0 {
            (health.current as f32 / health.max as f32 * 100.0).clamp(0.0, 100.0)
        } else {
            0.0
        };
        let slice = HealthBarSlice::for_percent(percent);

        if let Some(index) = sheet.get(slice.as_str()) {
            if let Some(atlas) = &mut image.texture_atlas {
                if atlas.index != index {
                    atlas.index = index;
                }
            }
        }

        for child in children.iter() {
            if let Ok(mut text) = text_query.get_mut(child) {
                let hp_str = format!("{} / {}", health.current, health.max);
                if **text != hp_str {
                    **text = hp_str;
                }
            }
        }
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
    let slice = health_bar_slice(percent);

    // Find sprite and text components in children
    if let Ok(bar_children) = children.get(bar_entity) {
        for child in bar_children.iter() {
            // Update sprite
            if let Ok(mut image_node) = sprite_query.get_mut(child) {
                if let Some(index) = sheet.get(slice.as_str()) {
                    if let Some(atlas) = &mut image_node.texture_atlas {
                        atlas.index = index;
                    }
                }
            }
            // Update text
            if let Ok(mut text) = text_query.get_mut(child) {
                **text = format!("{}/{}", current, max);
            }
        }
    }
}
