use bevy::prelude::*;

use crate::assets::{GameFonts, GameSprites, ItemDetailIconsSlice};
use crate::economy::WorthGold;
use crate::inventory::Inventory;
use crate::location::Store;
use crate::stats::StatType;
use crate::ui::widgets::{CentralDetailPanel, GoldDisplay};

use super::{InfoPanelSource, StoreInfoPanel};

/// System to populate the store info panel with item details.
pub fn populate_store_info_panel(
    mut commands: Commands,
    query: Query<(Entity, &StoreInfoPanel)>,
    inventory: Res<Inventory>,
    store: Res<Store>,
    game_fonts: Res<GameFonts>,
) {
    for (entity, panel) in &query {
        // Dark brown text color
        let text_color = TextColor(Color::srgb(0.4, 0.25, 0.15));

        match panel.source {
            InfoPanelSource::Store { selected_index } => {
                // Get the selected store item
                let Some(store_item) = store.inventory.get(selected_index) else {
                    continue;
                };

                // Get display item for stats/price
                let display_item = store_item.display_item();

                // Remove the marker and add children with item details
                commands
                    .entity(entity)
                    .remove::<StoreInfoPanel>()
                    .with_children(|parent| {
                        if let Some(item) = display_item {
                            // Item name
                            parent.spawn((
                                Text::new(&item.name),
                                game_fonts.pixel_font(24.0),
                                text_color,
                            ));

                            // Stats (only show non-zero stats)
                            for stat_type in StatType::all() {
                                let value = item.stats.value(*stat_type);
                                if value > 0 {
                                    let stat_name = match stat_type {
                                        StatType::Health => "HP",
                                        StatType::Attack => "ATK",
                                        StatType::Defense => "DEF",
                                        StatType::GoldFind => "Gold Find",
                                        StatType::Mining => "Mining",
                                        StatType::MagicFind => "Magic Find",
                                    };
                                    parent.spawn((
                                        Text::new(format!("{}: +{}", stat_name, value)),
                                        game_fonts.pixel_font(18.0),
                                        text_color,
                                    ));
                                }
                            }

                            // Cost with gold icon
                            parent.spawn(
                                GoldDisplay::new(item.purchase_price())
                                    .with_font_size(18.0)
                                    .with_color(text_color.0),
                            );
                        } else {
                            // Out of stock
                            parent.spawn((
                                Text::new("Out of Stock"),
                                game_fonts.pixel_font(18.0),
                                text_color,
                            ));
                        }
                    });
            }
            InfoPanelSource::Inventory { selected_index } => {
                // Get the selected item from inventory
                let inv_item = inventory.items.get(selected_index);

                // Remove the marker and add children with item details
                commands
                    .entity(entity)
                    .remove::<StoreInfoPanel>()
                    .with_children(|parent| {
                        if let Some(inv_item) = inv_item {
                            // Item name
                            parent.spawn((
                                Text::new(&inv_item.item.name),
                                game_fonts.pixel_font(24.0),
                                text_color,
                            ));

                            // Stats (only show non-zero stats)
                            for stat_type in StatType::all() {
                                let value = inv_item.item.stats.value(*stat_type);
                                if value > 0 {
                                    let stat_name = match stat_type {
                                        StatType::Health => "HP",
                                        StatType::Attack => "ATK",
                                        StatType::Defense => "DEF",
                                        StatType::GoldFind => "Gold Find",
                                        StatType::Mining => "Mining",
                                        StatType::MagicFind => "Magic Find",
                                    };
                                    parent.spawn((
                                        Text::new(format!("{}: +{}", stat_name, value)),
                                        game_fonts.pixel_font(18.0),
                                        text_color,
                                    ));
                                }
                            }

                            // Sell price
                            parent.spawn(
                                GoldDisplay::new(inv_item.item.sell_price())
                                    .with_font_size(18.0)
                                    .with_color(text_color.0),
                            );
                        } else {
                            parent.spawn((
                                Text::new("Empty"),
                                game_fonts.pixel_font(18.0),
                                text_color,
                            ));
                        }
                    });
            }
        }
    }
}

pub fn populate_central_detail_panel(
    mut commands: Commands,
    query: Query<(Entity, &CentralDetailPanel)>,
    inventory: Res<Inventory>,
    store: Res<Store>,
    game_sprites: Res<GameSprites>,
    game_fonts: Res<GameFonts>,
) {
    for (entity, panel) in &query {
        let text_color = TextColor(Color::srgb(0.4, 0.25, 0.15));

        match panel.source {
            InfoPanelSource::Store { selected_index } => {
                let Some(store_item) = store.inventory.get(selected_index) else {
                    continue;
                };

                let display_item = store_item.display_item();

                commands
                    .entity(entity)
                    .remove::<CentralDetailPanel>()
                    .with_children(|parent| {
                        if let Some(item) = display_item {
                            // Item name - top margin positions below decorative border
                            parent.spawn((
                                Text::new(&item.name),
                                game_fonts.pixel_font(18.0),
                                text_color,
                                Node {
                                    margin: UiRect {
                                        top: Val::Px(36.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                            ));

                            // Item quality
                            parent.spawn((
                                Text::new(item.quality.display_name()),
                                game_fonts.pixel_font(16.0),
                                TextColor(item.quality.color()),
                                Node {
                                    position_type: PositionType::Relative,
                                    ..default()
                                },
                            ));

                            // Stats - all use icon + value format
                            spawn_stats_with_icons(
                                parent,
                                StatType::all()
                                    .iter()
                                    .filter_map(|stat_type| {
                                        let value = item.stats.value(*stat_type);
                                        if value > 0 {
                                            Some((*stat_type, value))
                                        } else {
                                            None
                                        }
                                    })
                                    .collect::<Vec<_>>()
                                    .as_slice(),
                                &game_sprites,
                                &game_fonts,
                                text_color,
                            );

                            // Price with gold icon
                            spawn_price_row(
                                parent,
                                item.purchase_price(),
                                &game_sprites,
                                &game_fonts,
                                text_color,
                            );
                        } else {
                            parent.spawn((
                                Text::new("Out of Stock"),
                                game_fonts.pixel_font(18.0),
                                text_color,
                                Node {
                                    margin: UiRect {
                                        top: Val::Px(36.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                            ));
                        }
                    });
            }
            InfoPanelSource::Inventory { selected_index } => {
                let inv_item = inventory.items.get(selected_index);

                commands
                    .entity(entity)
                    .remove::<CentralDetailPanel>()
                    .with_children(|parent| {
                        if let Some(inv_item) = inv_item {
                            // Item name - top margin positions below decorative border
                            parent.spawn((
                                Text::new(&inv_item.item.name),
                                game_fonts.pixel_font(18.0),
                                text_color,
                                Node {
                                    margin: UiRect {
                                        top: Val::Px(36.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                            ));

                            // Item quality
                            parent.spawn((
                                Text::new(inv_item.item.quality.display_name()),
                                game_fonts.pixel_font(16.0),
                                TextColor(inv_item.item.quality.color()),
                                Node {
                                    position_type: PositionType::Relative,
                                    ..default()
                                },
                            ));

                            // Stats - all use icon + value format
                            spawn_stats_with_icons(
                                parent,
                                StatType::all()
                                    .iter()
                                    .filter_map(|stat_type| {
                                        let value = inv_item.item.stats.value(*stat_type);
                                        if value > 0 {
                                            Some((*stat_type, value))
                                        } else {
                                            None
                                        }
                                    })
                                    .collect::<Vec<_>>()
                                    .as_slice(),
                                &game_sprites,
                                &game_fonts,
                                text_color,
                            );

                            // Sell price with gold icon
                            spawn_price_row(
                                parent,
                                inv_item.item.sell_price(),
                                &game_sprites,
                                &game_fonts,
                                text_color,
                            );
                        } else {
                            parent.spawn((
                                Text::new("Empty"),
                                game_fonts.pixel_font(18.0),
                                text_color,
                                Node {
                                    margin: UiRect {
                                        top: Val::Px(36.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                            ));
                        }
                    });
            }
        }
    }
}

/// Spawn stat rows with icons for each non-zero stat.
fn spawn_stats_with_icons(
    parent: &mut ChildBuilder,
    stats: &[(StatType, i32)],
    game_sprites: &GameSprites,
    game_fonts: &GameFonts,
    text_color: TextColor,
) {
    for (stat_type, value) in stats {
        let icon_slice = ItemDetailIconsSlice::for_stat(*stat_type);
        parent
            .spawn(Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(4.0),
                ..default()
            })
            .with_children(|row| {
                if let Some(sheet) = game_sprites.get(icon_slice.sprite_sheet_key()) {
                    if let Some(img) = sheet.image_node(icon_slice.as_str()) {
                        row.spawn((img, Node::default()));
                    }
                }
                row.spawn((
                    Text::new(format!("{}", value)),
                    game_fonts.pixel_font(18.0),
                    text_color,
                ));
            });
    }
}

/// Spawn a price row with gold icon.
fn spawn_price_row(
    parent: &mut ChildBuilder,
    price: i32,
    game_sprites: &GameSprites,
    game_fonts: &GameFonts,
    text_color: TextColor,
) {
    let gold_slice = ItemDetailIconsSlice::GoldIcon;
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(4.0),
            ..default()
        })
        .with_children(|row| {
            if let Some(sheet) = game_sprites.get(gold_slice.sprite_sheet_key()) {
                if let Some(img) = sheet.image_node(gold_slice.as_str()) {
                    row.spawn((img, Node::default()));
                }
            }
            row.spawn((
                Text::new(format!("{}", price)),
                game_fonts.pixel_font(18.0),
                text_color,
            ));
        });
}
