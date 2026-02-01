use bevy::prelude::*;
use bon::Builder;

use crate::assets::GameFonts;

#[derive(Builder)]
pub struct OutlinedQuantityConfig {
    #[builder(default = 14.0)]
    pub font_size: f32,
    #[builder(default = Color::WHITE)]
    pub text_color: Color,
    #[builder(default = Color::BLACK)]
    pub outline_color: Color,
    #[builder(default = 2.0)]
    pub right: f32,
    #[builder(default = 0.0)]
    pub bottom: f32,
}

impl Default for OutlinedQuantityConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}

pub fn spawn_outlined_quantity_text<M: Bundle>(
    parent: &mut ChildSpawnerCommands,
    game_fonts: &GameFonts,
    quantity: u32,
    config: OutlinedQuantityConfig,
    marker: M,
) {
    let text = quantity.to_string();
    let font = game_fonts.pixel_font(config.font_size);

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

            text_container.spawn((
                Text::new(&text),
                font,
                TextColor(config.text_color),
            ));
        });
}

pub struct OutlinedTextPlugin;

impl Plugin for OutlinedTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_add_outlined_text);
    }
}

#[derive(Component, Builder)]
#[builder(on(String, into))]
pub struct OutlinedText {
    #[builder(start_fn)]
    pub text: String,
    #[builder(default = 16.0)]
    pub font_size: f32,
    #[builder(default = Color::WHITE)]
    pub text_color: Color,
    #[builder(default = Color::BLACK)]
    pub outline_color: Color,
    #[builder(default = 1.0)]
    pub outline_offset: f32,
}

fn on_add_outlined_text(
    trigger: On<Add, OutlinedText>,
    mut commands: Commands,
    query: Query<&OutlinedText>,
    game_fonts: Res<GameFonts>,
) {
    let entity = trigger.entity;
    let Ok(outlined) = query.get(entity) else {
        return;
    };

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
            let offsets = [
                (offset, 0.0),
                (-offset, 0.0),
                (0.0, offset),
                (0.0, -offset),
            ];

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
