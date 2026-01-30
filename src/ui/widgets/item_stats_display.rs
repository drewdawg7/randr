use bevy::prelude::*;
use bon::Builder;

use crate::assets::{GameFonts, GameSprites, ItemDetailIconsSlice};
use crate::stats::StatType;

pub struct ItemStatsDisplayPlugin;

impl Plugin for ItemStatsDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_add_item_stats_display);
    }
}

#[derive(Clone, Copy, Default)]
pub enum StatsDisplayMode {
    TextOnly,
    #[default]
    IconValue,
}

#[derive(Component, Builder)]
pub struct ItemStatsDisplay {
    #[builder(start_fn)]
    pub stats: Vec<(StatType, i32)>,
    pub comparison: Option<Vec<(StatType, i32)>>,
    #[builder(default = 18.0)]
    pub font_size: f32,
    #[builder(default = Color::srgb(0.4, 0.25, 0.15))]
    pub text_color: Color,
    #[builder(default)]
    pub mode: StatsDisplayMode,
}

fn on_add_item_stats_display(
    trigger: Trigger<OnAdd, ItemStatsDisplay>,
    mut commands: Commands,
    query: Query<&ItemStatsDisplay>,
    game_sprites: Res<GameSprites>,
    game_fonts: Res<GameFonts>,
) {
    let entity = trigger.entity();
    let Ok(display) = query.get(entity) else {
        return;
    };

    let stats = display.stats.clone();
    let comparison = display.comparison.clone();
    let font_size = display.font_size;
    let text_color = display.text_color;
    let mode = display.mode;

    commands
        .entity(entity)
        .remove::<ItemStatsDisplay>()
        .insert(Node {
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|parent| {
            for (stat_type, value) in stats {
                let delta = comparison.as_ref().map(|comp_stats| {
                    let equipped_value = comp_stats
                        .iter()
                        .find(|(t, _)| *t == stat_type)
                        .map(|(_, v)| *v)
                        .unwrap_or(0);
                    value - equipped_value
                });

                match mode {
                    StatsDisplayMode::TextOnly => {
                        parent
                            .spawn(Node {
                                flex_direction: FlexDirection::Row,
                                column_gap: Val::Px(4.0),
                                ..default()
                            })
                            .with_children(|row| {
                                row.spawn((
                                    Text::new(format!("{}: +{}", stat_type.display_name(), value)),
                                    game_fonts.pixel_font(font_size),
                                    TextColor(text_color),
                                ));
                                if let Some(d) = delta {
                                    if d != 0 {
                                        let (delta_text, delta_color) = if d > 0 {
                                            (format!("(+{})", d), Color::srgb(0.3, 0.8, 0.3))
                                        } else {
                                            (format!("({})", d), Color::srgb(0.8, 0.3, 0.3))
                                        };
                                        row.spawn((
                                            Text::new(delta_text),
                                            game_fonts.pixel_font(font_size),
                                            TextColor(delta_color),
                                        ));
                                    }
                                }
                            });
                    }
                    StatsDisplayMode::IconValue => {
                        let icon_slice = ItemDetailIconsSlice::for_stat(stat_type);
                        parent
                            .spawn(Node {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                column_gap: Val::Px(4.0),
                                ..default()
                            })
                            .with_children(|row| {
                                if let Some(sheet) =
                                    game_sprites.get(icon_slice.sprite_sheet_key())
                                {
                                    if let Some(img) = sheet.image_node(icon_slice.as_str()) {
                                        row.spawn((img, Node::default()));
                                    }
                                }
                                row.spawn((
                                    Text::new(format!("{}", value)),
                                    game_fonts.pixel_font(font_size),
                                    TextColor(text_color),
                                ));
                                if let Some(d) = delta {
                                    if d != 0 {
                                        let (delta_text, delta_color) = if d > 0 {
                                            (format!("(+{})", d), Color::srgb(0.3, 0.8, 0.3))
                                        } else {
                                            (format!("({})", d), Color::srgb(0.8, 0.3, 0.3))
                                        };
                                        row.spawn((
                                            Text::new(delta_text),
                                            game_fonts.pixel_font(font_size),
                                            TextColor(delta_color),
                                        ));
                                    }
                                }
                            });
                    }
                }
            }
        });
}
