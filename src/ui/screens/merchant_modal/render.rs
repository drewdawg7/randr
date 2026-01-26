use bevy::prelude::*;

use crate::assets::GameFonts;
use crate::economy::WorthGold;
use crate::inventory::{Inventory, InventoryItem, ManagesEquipment, ManagesItems};
use crate::stats::StatType;
use crate::ui::focus::{FocusPanel, FocusState};
use crate::ui::screens::modal::spawn_modal_overlay;
use crate::ui::screens::InfoPanelSource;
use crate::ui::widgets::{ItemDetailPane, ItemDetailPaneContent, ItemGrid, ItemGridEntry, ItemGridFocusPanel, ItemStatsDisplay, OutlinedText};

use super::state::{
    MerchantModalRoot, MerchantPlayerGrid, MerchantStock, MerchantStockGrid,
};

/// Sync system that reactively updates grids when inventory or stock changes.
/// Uses Bevy's native change detection via `is_changed()`.
pub fn sync_merchant_grids(
    inventory: Res<Inventory>,
    stock: Option<Res<MerchantStock>>,
    mut stock_grids: Query<&mut ItemGrid, (With<MerchantStockGrid>, Without<MerchantPlayerGrid>)>,
    mut player_grids: Query<&mut ItemGrid, (With<MerchantPlayerGrid>, Without<MerchantStockGrid>)>,
) {
    let Some(stock) = stock else {
        return;
    };

    // Only run if inventory or stock has changed
    if !inventory.is_changed() && !stock.is_changed() {
        return;
    }

    // Update stock grid if stock changed
    if stock.is_changed() {
        if let Ok(mut grid) = stock_grids.get_single_mut() {
            grid.items = get_merchant_stock_entries(&stock);
            if !grid.items.is_empty() {
                grid.selected_index = grid.selected_index.min(grid.items.len() - 1);
            } else {
                grid.selected_index = 0;
            }
        }
    }

    // Update player grid if inventory changed
    if inventory.is_changed() {
        if let Ok(mut grid) = player_grids.get_single_mut() {
            grid.items = get_player_inventory_entries(&inventory);
            if !grid.items.is_empty() {
                grid.selected_index = grid.selected_index.min(grid.items.len() - 1);
            } else {
                grid.selected_index = 0;
            }
        }
    }
}

/// Convert merchant stock to grid entries for display.
pub fn get_merchant_stock_entries(stock: &MerchantStock) -> Vec<ItemGridEntry> {
    stock
        .items
        .iter()
        .filter_map(|store_item| {
            store_item.display_item().map(|item| ItemGridEntry {
                sprite_sheet_key: item.item_id.sprite_sheet_key(),
                sprite_name: item.item_id.sprite_name().to_string(),
                quantity: store_item.quantity() as u32,
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
            sprite_sheet_key: inv_item.item.item_id.sprite_sheet_key(),
            sprite_name: inv_item.item.item_id.sprite_name().to_string(),
            quantity: inv_item.quantity,
        })
        .collect()
}

/// Get player inventory items as a vector.
pub fn get_player_inventory_items(inventory: &Inventory) -> Vec<&InventoryItem> {
    inventory.get_inventory_items().iter().collect()
}

/// Gets comparison stats from the equipped item in the same slot as the given item.
/// Returns None if the item is not equipment, or Some(empty vec) if slot is empty.
fn get_comparison_stats(
    item: &crate::item::Item,
    inventory: &Inventory,
) -> Option<Vec<(StatType, i32)>> {
    // Only compare equipment items
    let slot = item.item_type.equipment_slot()?;

    // Get equipped item stats (empty vec if slot is empty)
    let comparison: Vec<_> = inventory
        .get_equipped_item(slot)
        .map(|equipped| {
            equipped
                .item
                .stats
                .stats()
                .iter()
                .map(|(t, si)| (*t, si.current_value))
                .collect()
        })
        .unwrap_or_default();

    Some(comparison)
}

/// Spawn the merchant modal UI with stock grid, player inventory grid, and detail pane.
/// Called from RegisteredModal::spawn via run_system_cached.
pub fn spawn_merchant_modal_impl(
    mut commands: Commands,
    stock: &MerchantStock,
    inventory: &Inventory,
) {
    // Initialize focus on merchant stock grid
    commands.insert_resource(FocusState {
        focused: Some(FocusPanel::MerchantStock),
    });

    let stock_entries = get_merchant_stock_entries(stock);
    let player_entries = get_player_inventory_entries(inventory);

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
                        ItemGridFocusPanel(FocusPanel::MerchantStock),
                        ItemGrid {
                            items: stock_entries,
                            selected_index: 0,
                            grid_size: 5,
                        },
                    ));

                    // Player inventory grid (5x5)
                    row.spawn((
                        MerchantPlayerGrid,
                        ItemGridFocusPanel(FocusPanel::PlayerInventory),
                        ItemGrid {
                            items: player_entries,
                            selected_index: 0,
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
    focus_state: Option<Res<FocusState>>,
    stock_grids: Query<&ItemGrid, With<MerchantStockGrid>>,
    player_grids: Query<&ItemGrid, With<MerchantPlayerGrid>>,
    mut panes: Query<&mut ItemDetailPane>,
    content_query: Query<(Entity, Option<&Children>), With<ItemDetailPaneContent>>,
) {
    let Some(stock) = stock else {
        return;
    };

    let Some(focus_state) = focus_state else {
        return;
    };

    // Check for data changes that require refresh
    let data_changed = stock.is_changed() || inventory.is_changed();

    // Determine which grid is focused and build the appropriate source
    let source = if focus_state.is_focused(FocusPanel::MerchantStock) {
        if let Ok(grid) = stock_grids.get_single() {
            InfoPanelSource::Store { selected_index: grid.selected_index }
        } else {
            return;
        }
    } else if focus_state.is_focused(FocusPanel::PlayerInventory) {
        if let Ok(grid) = player_grids.get_single() {
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

    // Check if we need to update:
    // - First render (no children yet)
    // - Source changed (different selection or focus)
    // - Data changed (stock or inventory was modified)
    let needs_initial = children.is_none();
    if pane.source == source && !needs_initial && !data_changed {
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
        // Item name (quality-colored with black outline)
        parent.spawn(
            OutlinedText::new(&item.name)
                .with_font_size(16.0)
                .with_color(item.quality.color()),
        );

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

        // Stats display with comparison for equipment items
        let stats: Vec<_> = item
            .stats
            .stats()
            .iter()
            .map(|(t, si)| (*t, si.current_value))
            .collect();
        if !stats.is_empty() {
            let mut display = ItemStatsDisplay::from_stats_iter(stats)
                .with_font_size(14.0)
                .with_color(Color::srgb(0.85, 0.85, 0.85));

            // Add comparison for equipment items (both store and player inventory)
            if let Some(comparison) = get_comparison_stats(item, &inventory) {
                display = display.with_comparison(comparison);
            }

            parent.spawn(display);
        }
    });
}
