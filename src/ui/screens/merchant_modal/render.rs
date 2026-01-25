use bevy::prelude::*;

use crate::assets::GameFonts;
use crate::economy::WorthGold;
use crate::inventory::{Inventory, InventoryItem, ManagesItems};
use crate::ui::screens::modal::{spawn_modal_overlay, ActiveModal, ModalType};
use crate::ui::screens::InfoPanelSource;
use crate::ui::widgets::{ItemDetailPane, ItemDetailPaneContent, ItemGrid, ItemGridEntry, ItemStatsDisplay};

use super::state::{
    MerchantDetailRefresh, MerchantModalRoot, MerchantPlayerGrid, MerchantStock, MerchantStockGrid,
};

/// Convert merchant stock to grid entries for display.
pub fn get_merchant_stock_entries(stock: &MerchantStock) -> Vec<ItemGridEntry> {
    stock
        .items
        .iter()
        .filter_map(|store_item| {
            store_item.display_item().map(|item| ItemGridEntry {
                sprite_name: item.item_id.sprite_name().to_string(),
            })
        })
        .collect()
}

/// Convert player inventory items to grid entries for display.
pub fn get_player_inventory_entries(inventory: &Inventory) -> Vec<ItemGridEntry> {
    inventory
        .get_inventory_items()
        .iter()
        .map(|inv_item| ItemGridEntry {
            sprite_name: inv_item.item.item_id.sprite_name().to_string(),
        })
        .collect()
}

/// Get player inventory items as a vector.
pub fn get_player_inventory_items(inventory: &Inventory) -> Vec<&InventoryItem> {
    inventory.get_inventory_items().iter().collect()
}

/// Spawn the merchant modal UI with stock grid, player inventory grid, and detail pane.
pub fn spawn_merchant_modal(
    mut commands: Commands,
    stock: Res<MerchantStock>,
    inventory: Res<Inventory>,
    mut active_modal: ResMut<ActiveModal>,
) {
    commands.remove_resource::<super::state::SpawnMerchantModal>();
    active_modal.modal = Some(ModalType::MerchantModal);

    let stock_entries = get_merchant_stock_entries(&stock);
    let player_entries = get_player_inventory_entries(&inventory);

    let overlay = spawn_modal_overlay(&mut commands);
    commands
        .entity(overlay)
        .insert(MerchantModalRoot)
        .with_children(|parent| {
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(16.0),
                    align_items: AlignItems::FlexStart,
                    ..default()
                })
                .with_children(|row| {
                    // Merchant stock grid (5x5) - focused by default
                    row.spawn((
                        MerchantStockGrid,
                        ItemGrid {
                            items: stock_entries,
                            selected_index: 0,
                            is_focused: true,
                            grid_size: 5,
                        },
                    ));

                    // Player inventory grid (5x5)
                    row.spawn((
                        MerchantPlayerGrid,
                        ItemGrid {
                            items: player_entries,
                            selected_index: 0,
                            is_focused: false,
                            grid_size: 5,
                        },
                    ));

                    // Item detail pane
                    row.spawn(ItemDetailPane {
                        source: InfoPanelSource::Store { selected_index: 0 },
                    });
                });
        });
}

/// Populates the item detail pane with the currently selected item's information.
pub fn populate_merchant_detail_pane(
    mut commands: Commands,
    game_fonts: Res<GameFonts>,
    stock: Option<Res<MerchantStock>>,
    inventory: Res<Inventory>,
    refresh_trigger: Option<Res<MerchantDetailRefresh>>,
    stock_grids: Query<&ItemGrid, With<MerchantStockGrid>>,
    player_grids: Query<&ItemGrid, With<MerchantPlayerGrid>>,
    mut panes: Query<&mut ItemDetailPane>,
    content_query: Query<(Entity, Option<&Children>), With<ItemDetailPaneContent>>,
) {
    let Some(stock) = stock else {
        return;
    };

    // Consume the refresh trigger if present
    let force_refresh = refresh_trigger.is_some();
    if force_refresh {
        commands.remove_resource::<MerchantDetailRefresh>();
    }

    // Determine which grid is focused and build the appropriate source
    let source = if let Ok(grid) = stock_grids.get_single() {
        if grid.is_focused {
            InfoPanelSource::Store { selected_index: grid.selected_index }
        } else if let Ok(player_grid) = player_grids.get_single() {
            InfoPanelSource::Inventory { selected_index: player_grid.selected_index }
        } else {
            return;
        }
    } else if let Ok(grid) = player_grids.get_single() {
        if grid.is_focused {
            InfoPanelSource::Inventory { selected_index: grid.selected_index }
        } else {
            return;
        }
    } else {
        return;
    };

    let Ok(mut pane) = panes.get_single_mut() else {
        return;
    };

    let Ok((content_entity, children)) = content_query.get_single() else {
        return;
    };

    // Check if we need to update
    let needs_initial = children.is_none();
    if pane.source == source && !needs_initial && !force_refresh {
        return;
    }

    // Update pane source
    pane.source = source;

    // Despawn existing content children
    if let Some(children) = children {
        for &child in children.iter() {
            commands.entity(child).despawn_recursive();
        }
    }

    // Look up the selected item based on which grid is focused
    let item_info: Option<(&crate::item::Item, u32, Option<i32>)> = match source {
        InfoPanelSource::Store { selected_index } => {
            stock.items.get(selected_index).and_then(|store_item| {
                store_item.display_item().map(|item| {
                    let qty = store_item.quantity() as u32;
                    let price = item.purchase_price();
                    (item, qty, Some(price))
                })
            })
        }
        InfoPanelSource::Inventory { selected_index } => {
            get_player_inventory_items(&inventory)
                .get(selected_index)
                .map(|inv_item| {
                    let price = inv_item.item.sell_price();
                    (&inv_item.item, inv_item.quantity, Some(price))
                })
        }
        _ => None,
    };

    let Some((item, quantity, price)) = item_info else {
        return;
    };

    // Spawn item details
    commands.entity(content_entity).with_children(|parent| {
        // Item name (quality-colored)
        parent.spawn((
            Text::new(&item.name),
            game_fonts.pixel_font(16.0),
            TextColor(item.quality.color()),
        ));

        // Item type
        parent.spawn((
            Text::new(format!("{}", item.item_type)),
            game_fonts.pixel_font(14.0),
            TextColor(Color::srgb(0.7, 0.7, 0.7)),
        ));

        // Quality label
        parent.spawn((
            Text::new(item.quality.display_name()),
            game_fonts.pixel_font(14.0),
            TextColor(item.quality.color()),
        ));

        // Quantity
        if quantity > 1 {
            parent.spawn((
                Text::new(format!("Qty: {}", quantity)),
                game_fonts.pixel_font(14.0),
                TextColor(Color::srgb(0.3, 0.8, 0.3)),
            ));
        }

        // Price (with label based on context)
        if let Some(price) = price {
            let price_label = match source {
                InfoPanelSource::Store { .. } => format!("Price: {}g", price),
                InfoPanelSource::Inventory { .. } => format!("Sell: {}g", price),
                _ => format!("{}g", price),
            };
            parent.spawn((
                Text::new(price_label),
                game_fonts.pixel_font(14.0),
                TextColor(Color::srgb(0.9, 0.8, 0.2)),
            ));
        }

        // Stats display
        let stats: Vec<_> = item
            .stats
            .stats()
            .iter()
            .map(|(t, si)| (*t, si.current_value))
            .collect();
        if !stats.is_empty() {
            parent.spawn(
                ItemStatsDisplay::from_stats_iter(stats)
                    .with_font_size(14.0)
                    .with_color(Color::srgb(0.85, 0.85, 0.85)),
            );
        }
    });
}
