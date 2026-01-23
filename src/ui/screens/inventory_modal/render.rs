use bevy::prelude::*;

use crate::assets::GameFonts;
use crate::inventory::{EquipmentSlot, Inventory, InventoryItem, ManagesEquipment, ManagesItems};
use crate::ui::screens::modal::spawn_modal_overlay;
use crate::ui::screens::InfoPanelSource;
use crate::ui::widgets::{ItemDetailPane, ItemDetailPaneContent, ItemGrid, ItemGridEntry, ItemStatsDisplay};

use super::state::InventoryModalRoot;

/// Returns inventory items in display order: equipped items first, then backpack.
pub fn get_ordered_items(inventory: &Inventory) -> Vec<&InventoryItem> {
    let mut items = Vec::new();
    for slot in EquipmentSlot::all() {
        if let Some(inv_item) = inventory.get_equipped_item(*slot) {
            items.push(inv_item);
        }
    }
    for inv_item in inventory.get_inventory_items() {
        items.push(inv_item);
    }
    items
}

/// Spawn the inventory modal UI with an ItemGrid and ItemDetailPane side by side.
pub fn spawn_inventory_modal(commands: &mut Commands, inventory: &Inventory) {
    let ordered = get_ordered_items(inventory);
    let items: Vec<ItemGridEntry> = ordered
        .iter()
        .map(|inv_item| ItemGridEntry {
            sprite_name: inv_item.item.item_id.sprite_name().to_string(),
        })
        .collect();

    let overlay = spawn_modal_overlay(commands);
    commands
        .entity(overlay)
        .insert(InventoryModalRoot)
        .with_children(|parent| {
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(16.0),
                    align_items: AlignItems::FlexStart,
                    ..default()
                })
                .with_children(|row| {
                    row.spawn(ItemGrid {
                        items,
                        selected_index: 0,
                        is_focused: true,
                    });
                    row.spawn(ItemDetailPane {
                        source: InfoPanelSource::Inventory { selected_index: 0 },
                    });
                });
        });
}

/// Populates the item detail pane with the currently selected item's information.
/// Runs when ItemGrid selection changes or when the content container is first created.
pub fn populate_item_detail_pane(
    mut commands: Commands,
    game_fonts: Res<GameFonts>,
    inventory: Res<Inventory>,
    grids: Query<&ItemGrid>,
    mut panes: Query<&mut ItemDetailPane>,
    content_query: Query<(Entity, Option<&Children>), With<ItemDetailPaneContent>>,
) {
    let Ok(grid) = grids.get_single() else {
        return;
    };

    let Ok(mut pane) = panes.get_single_mut() else {
        return;
    };

    let Ok((content_entity, children)) = content_query.get_single() else {
        return;
    };

    // Check if we need to update: source index differs from grid selection
    let current_source_index = match pane.source {
        InfoPanelSource::Inventory { selected_index } => selected_index,
        InfoPanelSource::Store { selected_index } => selected_index,
    };
    let needs_initial = children.is_none();
    if current_source_index == grid.selected_index && !needs_initial {
        return;
    }

    // Update pane source to match grid selection
    pane.source = InfoPanelSource::Inventory {
        selected_index: grid.selected_index,
    };

    // Despawn existing content children
    if let Some(children) = children {
        for &child in children.iter() {
            commands.entity(child).despawn_recursive();
        }
    }

    // Look up the selected item
    let ordered = get_ordered_items(&inventory);
    let Some(inv_item) = ordered.get(grid.selected_index) else {
        return;
    };

    let item = &inv_item.item;

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
