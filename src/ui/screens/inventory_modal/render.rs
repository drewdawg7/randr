use bevy::prelude::*;

use crate::inventory::{EquipmentSlot, Inventory, InventoryItem, ManagesEquipment, ManagesItems};
use crate::player::PlayerMarker;
use crate::ui::focus::FocusPanel;
use crate::ui::modal_content_row;
use crate::ui::InfoPanelSource;
use crate::ui::widgets::{
    ItemDetailDisplay, ItemDetailPane, ItemDetailPaneContent, ItemGrid, ItemGridEntry,
    ItemGridFocusPanel, ItemGridSelection,
};
use crate::ui::{FocusState, Modal, ModalBackground, SpawnModalExt};

use super::state::{BackpackGrid, EquipmentGrid, InventoryModalRoot};

pub fn sync_inventory_to_grids(
    player: Query<&Inventory, (With<PlayerMarker>, Changed<Inventory>)>,
    mut equipment_grids: Query<
        (&mut ItemGrid, &mut ItemGridSelection),
        (With<EquipmentGrid>, Without<BackpackGrid>),
    >,
    mut backpack_grids: Query<
        (&mut ItemGrid, &mut ItemGridSelection),
        (With<BackpackGrid>, Without<EquipmentGrid>),
    >,
) {
    let Ok(inventory) = player.single() else {
        return;
    };

    if let Ok((mut eq_grid, mut eq_selection)) = equipment_grids.single_mut() {
        eq_grid.items = get_equipment_items(inventory)
            .iter()
            .map(|inv_item| ItemGridEntry::from_inventory_item(inv_item))
            .collect();
        eq_selection.clamp(eq_grid.items.len());
    }

    if let Ok((mut bp_grid, mut bp_selection)) = backpack_grids.single_mut() {
        bp_grid.items = ItemGridEntry::from_inventory(inventory);
        bp_selection.clamp(bp_grid.items.len());
    }
}

/// Returns equipment items in slot order. Each entry corresponds to an EquipmentSlot.
/// Only populated slots produce entries.
pub fn get_equipment_items(inventory: &Inventory) -> Vec<&InventoryItem> {
    EquipmentSlot::all()
        .iter()
        .filter_map(|slot| inventory.get_equipped_item(*slot))
        .collect()
}

/// Returns backpack (non-equipped) items.
pub fn get_backpack_items(inventory: &Inventory) -> Vec<&InventoryItem> {
    inventory.get_inventory_items().iter().collect()
}

/// Spawn the inventory modal UI with an equipment grid, backpack grid, and detail pane.
pub fn spawn_inventory_modal(commands: &mut Commands, inventory: &Inventory) {
    commands.insert_resource(FocusState {
        focused: Some(FocusPanel::EquipmentGrid),
    });

    let equipment_entries: Vec<ItemGridEntry> = get_equipment_items(inventory)
        .iter()
        .map(|inv_item| ItemGridEntry::from_inventory_item(inv_item))
        .collect();

    let backpack_entries: Vec<ItemGridEntry> = ItemGridEntry::from_inventory(inventory);

    commands.spawn_modal(
        Modal::builder()
            .background(ModalBackground::None)
            .root_marker(Box::new(|e| {
                e.insert(InventoryModalRoot);
            }))
            .content(Box::new(move |c| {
                c.spawn(modal_content_row()).with_children(|row| {
                    row.spawn((
                        EquipmentGrid,
                        ItemGridFocusPanel(FocusPanel::EquipmentGrid),
                        ItemGrid {
                            items: equipment_entries,
                            grid_size: 3,
                        },
                        ItemGridSelection::default(),
                    ));
                    row.spawn((
                        BackpackGrid,
                        ItemGridFocusPanel(FocusPanel::BackpackGrid),
                        ItemGrid {
                            items: backpack_entries,
                            grid_size: 4,
                        },
                        ItemGridSelection::default(),
                    ));
                    row.spawn(ItemDetailPane {
                        source: InfoPanelSource::Equipment { selected_index: 0 },
                    });
                });
            }))
            .build(),
    );
}

pub fn populate_inventory_detail_pane_content(
    mut commands: Commands,
    player: Query<&Inventory, With<PlayerMarker>>,
    panes: Query<Ref<ItemDetailPane>>,
    content_query: Query<(Entity, Option<&Children>), With<ItemDetailPaneContent>>,
) {
    let Ok(inventory) = player.single() else {
        return;
    };

    for pane in &panes {
        if !pane.is_changed() {
            continue;
        }

        let Ok((content_entity, children)) = content_query.single() else {
            continue;
        };

        if let Some(children) = children {
            for child in children.iter() {
                commands.entity(child).despawn();
            }
        }

        let inv_item = match pane.source {
            InfoPanelSource::Equipment { selected_index } => {
                get_equipment_items(inventory).into_iter().nth(selected_index)
            }
            InfoPanelSource::Inventory { selected_index } => {
                get_backpack_items(inventory).into_iter().nth(selected_index)
            }
            _ => None,
        };

        let Some(inv_item) = inv_item else {
            continue;
        };

        let item = &inv_item.item;
        let comparison = if matches!(pane.source, InfoPanelSource::Inventory { .. }) {
            inventory.get_comparison_stats(item)
        } else {
            None
        };

        commands.entity(content_entity).with_children(|parent| {
            parent.spawn(
                ItemDetailDisplay::builder(item)
                    .quantity(inv_item.quantity)
                    .maybe_comparison(comparison)
                    .build(),
            );
        });
    }
}
