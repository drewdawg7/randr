use bevy::prelude::*;

use crate::assets::GameFonts;
use crate::economy::WorthGold;
use crate::inventory::Inventory;
use crate::item::Item;
use crate::location::Store;
use crate::stats::StatType;
use crate::ui::widgets::{CentralDetailPanel, GoldDisplay, ItemStatsDisplay};

use super::InfoPanelSource;

/// Extracts an item reference from the panel source.
fn get_item_from_source<'a>(
    source: &InfoPanelSource,
    store: &'a Store,
    inventory: &'a Inventory,
) -> Option<&'a Item> {
    match source {
        InfoPanelSource::Store { selected_index } => {
            store.inventory.get(*selected_index)?.display_item()
        }
        InfoPanelSource::Inventory { selected_index } => {
            inventory.items.get(*selected_index).map(|i| &i.item)
        }
    }
}

/// Returns the appropriate price for the item based on the source.
fn get_price_for_source(source: &InfoPanelSource, item: &Item) -> i32 {
    match source {
        InfoPanelSource::Store { .. } => item.purchase_price(),
        InfoPanelSource::Inventory { .. } => item.sell_price(),
    }
}

pub fn populate_central_detail_panel(
    mut commands: Commands,
    query: Query<(Entity, &CentralDetailPanel)>,
    inventory: Res<Inventory>,
    store: Res<Store>,
    game_fonts: Res<GameFonts>,
) {
    for (entity, panel) in &query {
        let text_color = TextColor(Color::srgb(0.4, 0.25, 0.15));
        let item = get_item_from_source(&panel.source, &store, &inventory);

        commands
            .entity(entity)
            .remove::<CentralDetailPanel>()
            .with_children(|parent| {
                if let Some(item) = item {
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

                    // Stats - icon + value format
                    parent.spawn(
                        ItemStatsDisplay::from_stats_iter(
                            StatType::all()
                                .iter()
                                .map(|st| (*st, item.stats.value(*st))),
                        )
                        .icon_value()
                        .with_color(text_color.0),
                    );

                    // Price with gold icon
                    parent.spawn(
                        GoldDisplay::new(get_price_for_source(&panel.source, item))
                            .with_font_size(18.0)
                            .with_color(text_color.0),
                    );
                } else {
                    // Empty or out of stock
                    let message = match panel.source {
                        InfoPanelSource::Store { .. } => "Out of Stock",
                        InfoPanelSource::Inventory { .. } => "Empty",
                    };
                    parent.spawn((
                        Text::new(message),
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
