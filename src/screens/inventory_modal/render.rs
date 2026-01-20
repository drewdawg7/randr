use bevy::prelude::*;

use crate::inventory::Inventory;
use crate::screens::modal::{spawn_modal_overlay, ActiveModal, ModalType};
use crate::ui::widgets::StatRow;
use crate::ui::{inventory_selection_bg, spawn_modal_hint, UiText};

use super::state::{InventoryItemUI, InventoryModalRoot, InventorySelection, ItemInfo};
use super::utils::get_all_inventory_items;

/// Spawn the inventory modal UI.
pub fn spawn_inventory_modal(
    commands: &mut Commands,
    inventory: &Inventory,
    selection: &mut InventorySelection,
) {
    let items = get_all_inventory_items(inventory);
    selection.set_count(items.len());

    let overlay = spawn_modal_overlay(commands);

    commands
        .entity(overlay)
        .insert(InventoryModalRoot)
        .with_children(|parent| {
            // Modal content container with wider width for two-panel layout
            parent
                .spawn((
                    Node {
                        width: Val::Px(1000.0),
                        max_width: Val::Percent(90.0),
                        height: Val::Px(700.0),
                        max_height: Val::Percent(90.0),
                        flex_direction: FlexDirection::Row,
                        padding: UiRect::all(Val::Px(30.0)),
                        border: UiRect::all(Val::Px(3.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.12, 0.1)),
                    BorderColor(Color::srgb(0.6, 0.5, 0.3)),
                ))
                .with_children(|modal| {
                    // Left panel: Item list
                    spawn_item_list_panel(modal, &items, selection.index);

                    // Right panel: Item details
                    spawn_item_details_panel(modal, items.get(selection.index));
                });
        });
}

/// Spawn the left panel with the item list.
fn spawn_item_list_panel(parent: &mut ChildBuilder, items: &[ItemInfo], selected_index: usize) {
    parent
        .spawn(Node {
            width: Val::Percent(50.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::right(Val::Px(20.0)),
            border: UiRect::right(Val::Px(2.0)),
            ..default()
        })
        .with_children(|panel| {
            // Title
            panel.spawn(UiText::title("Inventory").build_with_node());

            // Item list container with scrolling
            panel
                .spawn(Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(6.0),
                    overflow: Overflow::clip_y(),
                    flex_grow: 1.0,
                    ..default()
                })
                .with_children(|list| {
                    if items.is_empty() {
                        list.spawn((
                            Text::new("No items in inventory"),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.6, 0.6, 0.6)),
                        ));
                    } else {
                        for (index, item_info) in items.iter().enumerate() {
                            spawn_item_row(list, item_info, index, index == selected_index);
                        }
                    }
                });

            // Instructions at bottom
            panel
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(5.0),
                    margin: UiRect::top(Val::Px(15.0)),
                    ..default()
                })
                .with_children(|instructions| {
                    spawn_modal_hint(instructions, "↑↓: Navigate  Enter: Equip/Unequip");
                    spawn_modal_hint(instructions, "I/Esc: Close");
                });
        });
}

/// Spawn a single item row in the list.
fn spawn_item_row(parent: &mut ChildBuilder, item_info: &ItemInfo, index: usize, is_selected: bool) {
    let item = item_info.item();
    let bg_color = inventory_selection_bg(is_selected);

    parent
        .spawn((
            InventoryItemUI { index },
            Node {
                padding: UiRect::all(Val::Px(8.0)),
                width: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(bg_color),
        ))
        .with_children(|row| {
            // Item name with quality color
            let quality_color = item.quality.color();
            let equipped_marker = if item_info.is_equipped() { "[E] " } else { "" };
            let quantity_text = if item_info.quantity() > 1 {
                format!(" (x{})", item_info.quantity())
            } else {
                String::new()
            };

            row.spawn((
                Text::new(format!(
                    "{}{}{}",
                    equipped_marker, item.name, quantity_text
                )),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(quality_color),
            ));
        });
}

/// Spawn the right panel with item details.
fn spawn_item_details_panel(parent: &mut ChildBuilder, item_info: Option<&ItemInfo>) {
    parent
        .spawn(Node {
            width: Val::Percent(50.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::left(Val::Px(20.0)),
            ..default()
        })
        .with_children(|panel| {
            // Title
            panel.spawn(UiText::title("Details").build_with_node());

            if let Some(item_info) = item_info {
                let item = item_info.item();

                // Item name with quality color
                panel.spawn((
                    Text::new(&item.name),
                    TextFont {
                        font_size: 26.0,
                        ..default()
                    },
                    TextColor(item.quality.color()),
                    Node {
                        margin: UiRect::bottom(Val::Px(15.0)),
                        ..default()
                    },
                ));

                // Quality
                panel.spawn(
                    StatRow::new("Quality:", item.quality.display_name())
                        .label_width(100.0)
                        .label_color(Color::srgb(0.7, 0.7, 0.7))
                        .value_color(Color::srgb(0.9, 0.9, 0.9))
                        .bottom_margin(8.0),
                );

                // Item type
                panel.spawn(
                    StatRow::new("Type:", item.item_type.to_string())
                        .label_width(100.0)
                        .label_color(Color::srgb(0.7, 0.7, 0.7))
                        .value_color(Color::srgb(0.8, 0.8, 0.8))
                        .bottom_margin(8.0),
                );

                // Value
                panel.spawn(
                    StatRow::new("Value:", format!("{} gold", item.gold_value))
                        .label_width(100.0)
                        .label_color(Color::srgb(0.7, 0.7, 0.7))
                        .value_color(Color::srgb(1.0, 0.84, 0.0))
                        .bottom_margin(8.0),
                );

                // Upgrades (for equipment)
                if item.item_type.is_equipment() {
                    panel.spawn(
                        StatRow::new("Upgrades:", format!("{} / {}", item.num_upgrades, item.max_upgrades))
                            .label_width(100.0)
                            .label_color(Color::srgb(0.7, 0.7, 0.7))
                            .value_color(Color::srgb(0.7, 0.9, 1.0))
                            .bottom_margin(8.0),
                    );
                }

                // Separator
                panel.spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(2.0),
                    margin: UiRect::vertical(Val::Px(15.0)),
                    ..default()
                });

                // Stats section
                if !item.stats.stats().is_empty() {
                    panel.spawn(UiText::section("Stats:").build_with_node());

                    for (stat_type, stat_instance) in item.stats.stats() {
                        if stat_instance.current_value > 0 {
                            panel.spawn((
                                Text::new(format!(
                                    "  {:?}: +{}",
                                    stat_type, stat_instance.current_value
                                )),
                                TextFont {
                                    font_size: 20.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.5, 1.0, 0.5)),
                                Node {
                                    margin: UiRect::bottom(Val::Px(5.0)),
                                    ..default()
                                },
                            ));
                        }
                    }
                }

                // Locked status
                if item.is_locked {
                    panel.spawn((
                        Text::new("[LOCKED]"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 0.3, 0.3)),
                        Node {
                            margin: UiRect::top(Val::Px(15.0)),
                            ..default()
                        },
                    ));
                }
            } else {
                panel.spawn((
                    Text::new("No item selected"),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.6, 0.6, 0.6)),
                ));
            }
        });
}

/// System to update the inventory display when player inventory or selection changes.
pub fn update_inventory_display(
    mut commands: Commands,
    modal_root: Query<Entity, With<InventoryModalRoot>>,
    active_modal: Res<ActiveModal>,
    inventory: Res<Inventory>,
    mut selection: ResMut<InventorySelection>,
) {
    // Only update if the inventory modal is open
    if active_modal.modal != Some(ModalType::Inventory) {
        return;
    }

    // Despawn and respawn the modal to update it
    if let Ok(entity) = modal_root.get_single() {
        commands.entity(entity).despawn_recursive();

        // Respawn with updated data
        spawn_inventory_modal(&mut commands, &inventory, &mut selection);
    }
}
