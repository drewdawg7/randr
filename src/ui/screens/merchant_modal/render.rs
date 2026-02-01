use bevy::prelude::*;

use crate::economy::WorthGold;
use crate::inventory::{Inventory, ManagesEquipment, ManagesItems};
use crate::ui::focus::{FocusPanel, FocusState};
use crate::ui::modal_content_row;
use crate::ui::InfoPanelSource;
use crate::ui::widgets::{
    ItemDetailDisplay, ItemDetailPane, ItemDetailPaneContent, ItemGrid, ItemGridEntry,
    ItemGridFocusPanel, PriceDisplay,
};
use crate::ui::{Modal, ModalBackground, SpawnModalExt};

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
            grid.items = ItemGridEntry::from_inventory(&inventory);
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


/// Spawn the merchant modal UI with stock grid, player inventory grid, and detail pane.
/// Called from RegisteredModal::spawn via run_system_cached.
pub fn spawn_merchant_modal_impl(
    mut commands: Commands,
    stock: &MerchantStock,
    inventory: &Inventory,
) {
    commands.insert_resource(FocusState {
        focused: Some(FocusPanel::MerchantStock),
    });

    let stock_entries = get_merchant_stock_entries(stock);
    let player_entries = ItemGridEntry::from_inventory(inventory);

    commands.spawn_modal(
        Modal::builder()
            .background(ModalBackground::None)
            .root_marker(Box::new(|e| {
                e.insert(MerchantModalRoot);
            }))
            .content(Box::new(move |c| {
                c.spawn(modal_content_row()).with_children(|row| {
                    row.spawn((
                        MerchantStockGrid,
                        ItemGridFocusPanel(FocusPanel::MerchantStock),
                        ItemGrid {
                            items: stock_entries,
                            selected_index: 0,
                            grid_size: 5,
                        },
                    ));
                    row.spawn((
                        MerchantPlayerGrid,
                        ItemGridFocusPanel(FocusPanel::PlayerInventory),
                        ItemGrid {
                            items: player_entries,
                            selected_index: 0,
                            grid_size: 5,
                        },
                    ));
                    row.spawn(ItemDetailPane {
                        source: InfoPanelSource::Store { selected_index: 0 },
                    });
                });
            }))
            .build(),
    );
}

/// Updates the detail pane source based on which grid is focused and selected.
/// Only runs when focus or grid selection changes.
pub fn update_merchant_detail_pane_source(
    focus_state: Option<Res<FocusState>>,
    stock_grids: Query<Ref<ItemGrid>, With<MerchantStockGrid>>,
    player_grids: Query<Ref<ItemGrid>, With<MerchantPlayerGrid>>,
    mut panes: Query<&mut ItemDetailPane>,
) {
    let Some(focus_state) = focus_state else {
        return;
    };

    // Check if focus or any grid changed
    let focus_changed = focus_state.is_changed();
    let stock_grid_changed = stock_grids
        .get_single()
        .map(|g| g.is_changed())
        .unwrap_or(false);
    let player_grid_changed = player_grids
        .get_single()
        .map(|g| g.is_changed())
        .unwrap_or(false);

    if !focus_changed && !stock_grid_changed && !player_grid_changed {
        return;
    }

    // Determine source from focused grid
    let source = if focus_state.is_focused(FocusPanel::MerchantStock) {
        stock_grids
            .get_single()
            .ok()
            .map(|g| InfoPanelSource::Store {
                selected_index: g.selected_index,
            })
    } else if focus_state.is_focused(FocusPanel::PlayerInventory) {
        player_grids
            .get_single()
            .ok()
            .map(|g| InfoPanelSource::Inventory {
                selected_index: g.selected_index,
            })
    } else {
        None
    };

    let Some(source) = source else {
        return;
    };

    // Update pane source (only if different to avoid unnecessary Changed trigger)
    for mut pane in &mut panes {
        if pane.source != source {
            pane.source = source;
        }
    }
}

pub fn populate_merchant_detail_pane_content(
    mut commands: Commands,
    stock: Option<Res<MerchantStock>>,
    inventory: Res<Inventory>,
    panes: Query<Ref<ItemDetailPane>>,
    content_query: Query<(Entity, Option<&Children>), With<ItemDetailPaneContent>>,
) {
    let Some(stock) = stock else {
        return;
    };

    let data_changed = stock.is_changed() || inventory.is_changed();

    for pane in &panes {
        if !pane.is_changed() && !data_changed {
            continue;
        }

        let Ok((content_entity, children)) = content_query.get_single() else {
            continue;
        };

        if let Some(children) = children {
            for &child in children.iter() {
                commands.entity(child).despawn_recursive();
            }
        }

        let item_info: Option<(&crate::item::Item, u32, PriceDisplay)> = match pane.source {
            InfoPanelSource::Store { selected_index } => {
                stock.items.get(selected_index).and_then(|store_item| {
                    store_item.display_item().map(|item| {
                        let qty = store_item.quantity() as u32;
                        let price = PriceDisplay::Buy(item.purchase_price());
                        (item, qty, price)
                    })
                })
            }
            InfoPanelSource::Inventory { selected_index } => inventory
                .get_inventory_items()
                .get(selected_index)
                .map(|inv_item| {
                    let price = PriceDisplay::Sell(inv_item.item.sell_price());
                    (&inv_item.item, inv_item.quantity, price)
                }),
            _ => None,
        };

        let Some((item, quantity, price)) = item_info else {
            continue;
        };

        let comparison = inventory.get_comparison_stats(item);

        commands.entity(content_entity).with_children(|parent| {
            parent.spawn(
                ItemDetailDisplay::builder(item)
                    .quantity(quantity)
                    .price(price)
                    .maybe_comparison(comparison)
                    .build(),
            );
        });
    }
}
