use bevy::prelude::*;

use crate::game::Player;
use crate::ui::inventory_selection_bg;
use crate::input::GameAction;
use crate::inventory::{EquipmentSlot, FindsItems, InventoryItem, ManagesEquipment, ManagesItems};
use crate::item::{Item, ItemType};
use super::modal::{spawn_modal_overlay, ActiveModal, ModalType};

/// Plugin that manages the inventory modal system.
pub struct InventoryModalPlugin;

impl Plugin for InventoryModalPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InventorySelection>().add_systems(
            Update,
            (
                handle_inventory_modal_toggle,
                handle_inventory_modal_input,
                update_inventory_display,
            ),
        );
    }
}

/// Component marker for the inventory modal UI.
#[derive(Component)]
pub struct InventoryModalRoot;

/// Component for individual inventory item UI elements.
#[derive(Component)]
struct InventoryItemUI {
    index: usize,
}

/// Resource for tracking which item is selected in the inventory.
#[derive(Resource, Default)]
struct InventorySelection {
    index: usize,
    count: usize,
}

impl InventorySelection {
    fn up(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        }
    }

    fn down(&mut self) {
        if self.index + 1 < self.count {
            self.index += 1;
        }
    }

    fn reset(&mut self) {
        self.index = 0;
    }

    fn set_count(&mut self, count: usize) {
        self.count = count;
        if self.index >= count && count > 0 {
            self.index = count - 1;
        }
    }
}

/// Information about an item in the inventory display.
#[derive(Clone)]
enum ItemInfo {
    Equipped(EquipmentSlot, Item),
    Backpack(uuid::Uuid, InventoryItem),
}

impl ItemInfo {
    fn item(&self) -> &Item {
        match self {
            ItemInfo::Equipped(_, item) => item,
            ItemInfo::Backpack(_, inv_item) => &inv_item.item,
        }
    }

    fn quantity(&self) -> u32 {
        match self {
            ItemInfo::Equipped(_, _) => 1,
            ItemInfo::Backpack(_, inv_item) => inv_item.quantity,
        }
    }

    fn is_equipped(&self) -> bool {
        matches!(self, ItemInfo::Equipped(_, _))
    }
}

/// Get all items for display (equipped first, then backpack).
fn get_all_inventory_items(player: &Player) -> Vec<ItemInfo> {
    let mut items = Vec::new();

    // Add equipped items first
    for slot in EquipmentSlot::all() {
        if let Some(inv_item) = player.get_equipped_item(*slot) {
            items.push(ItemInfo::Equipped(*slot, inv_item.item.clone()));
        }
    }

    // Add backpack items
    for inv_item in player.get_inventory_items() {
        items.push(ItemInfo::Backpack(inv_item.item.item_uuid, inv_item.clone()));
    }

    items
}

/// System to handle opening/closing the inventory modal with 'i' key.
fn handle_inventory_modal_toggle(
    mut commands: Commands,
    mut action_reader: EventReader<GameAction>,
    mut active_modal: ResMut<ActiveModal>,
    mut selection: ResMut<InventorySelection>,
    player: Res<Player>,
    existing_modal: Query<Entity, With<InventoryModalRoot>>,
) {
    for action in action_reader.read() {
        match action {
            GameAction::OpenInventory => {
                // Toggle: close if open, open if closed
                if let Ok(entity) = existing_modal.get_single() {
                    commands.entity(entity).despawn_recursive();
                    active_modal.modal = None;
                } else {
                    selection.reset();
                    spawn_inventory_modal(&mut commands, &player, &mut selection);
                    active_modal.modal = Some(ModalType::Inventory);
                }
            }
            GameAction::CloseModal => {
                // Close if this modal is open
                if active_modal.modal == Some(ModalType::Inventory) {
                    if let Ok(entity) = existing_modal.get_single() {
                        commands.entity(entity).despawn_recursive();
                        active_modal.modal = None;
                    }
                }
            }
            _ => {}
        }
    }
}

/// System to handle input when inventory modal is open.
fn handle_inventory_modal_input(
    mut action_reader: EventReader<GameAction>,
    active_modal: Res<ActiveModal>,
    mut selection: ResMut<InventorySelection>,
    mut player: ResMut<Player>,
) {
    if active_modal.modal != Some(ModalType::Inventory) {
        return;
    }

    for action in action_reader.read() {
        match action {
            GameAction::Navigate(dir) => {
                use crate::input::NavigationDirection;
                match dir {
                    NavigationDirection::Up => selection.up(),
                    NavigationDirection::Down => selection.down(),
                    _ => {}
                }
            }
            GameAction::Select => {
                // Equip/unequip the selected item
                toggle_equip(&mut player, &selection);
            }
            _ => {}
        }
    }
}

/// Toggle equipping/unequipping of the selected item.
fn toggle_equip(player: &mut Player, selection: &InventorySelection) {
    let items = get_all_inventory_items(player);
    if let Some(item_info) = items.get(selection.index) {
        match item_info {
            ItemInfo::Equipped(slot, _) => {
                // Unequip
                let _ = player.unequip_item(*slot);
            }
            ItemInfo::Backpack(item_uuid, _) => {
                // Try to equip
                if let Some(inv_item) = player.find_item_by_uuid(*item_uuid) {
                    if let Some(slot) = inv_item.item.item_type.equipment_slot() {
                        // Equip from inventory using the trait method
                        player.equip_from_inventory(*item_uuid, slot);
                    }
                }
            }
        }
    }
}

/// Spawn the inventory modal UI.
fn spawn_inventory_modal(
    commands: &mut Commands,
    player: &Player,
    selection: &mut InventorySelection,
) {
    let items = get_all_inventory_items(player);
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
fn spawn_item_list_panel(
    parent: &mut ChildBuilder,
    items: &[ItemInfo],
    selected_index: usize,
) {
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
            panel.spawn((
                Text::new("Inventory"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::srgb(0.95, 0.9, 0.7)),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ));

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
                    instructions.spawn((
                        Text::new("↑↓: Navigate  Enter: Equip/Unequip"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                    ));
                    instructions.spawn((
                        Text::new("I/Esc: Close"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                    ));
                });
        });
}

/// Spawn a single item row in the list.
fn spawn_item_row(
    parent: &mut ChildBuilder,
    item_info: &ItemInfo,
    index: usize,
    is_selected: bool,
) {
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
            let quality_color = get_quality_color(&item.quality);
            let equipped_marker = if item_info.is_equipped() { "[E] " } else { "" };
            let quantity_text = if item_info.quantity() > 1 {
                format!(" (x{})", item_info.quantity())
            } else {
                String::new()
            };

            row.spawn((
                Text::new(format!("{}{}{}", equipped_marker, item.name, quantity_text)),
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
            panel.spawn((
                Text::new("Details"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::srgb(0.95, 0.9, 0.7)),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ));

            if let Some(item_info) = item_info {
                let item = item_info.item();

                // Item name with quality color
                panel.spawn((
                    Text::new(&item.name),
                    TextFont {
                        font_size: 26.0,
                        ..default()
                    },
                    TextColor(get_quality_color(&item.quality)),
                    Node {
                        margin: UiRect::bottom(Val::Px(15.0)),
                        ..default()
                    },
                ));

                // Quality
                spawn_detail_row(
                    panel,
                    "Quality:",
                    item.quality.display_name(),
                    Color::srgb(0.9, 0.9, 0.9),
                );

                // Item type
                spawn_detail_row(
                    panel,
                    "Type:",
                    &format_item_type(&item.item_type),
                    Color::srgb(0.8, 0.8, 0.8),
                );

                // Value
                spawn_detail_row(
                    panel,
                    "Value:",
                    &format!("{} gold", item.gold_value),
                    Color::srgb(1.0, 0.84, 0.0),
                );

                // Upgrades (for equipment)
                if item.item_type.is_equipment() {
                    spawn_detail_row(
                        panel,
                        "Upgrades:",
                        &format!("{} / {}", item.num_upgrades, item.max_upgrades),
                        Color::srgb(0.7, 0.9, 1.0),
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
                    panel.spawn((
                        Text::new("Stats:"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.8, 0.5)),
                        Node {
                            margin: UiRect::bottom(Val::Px(10.0)),
                            ..default()
                        },
                    ));

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

/// Helper to spawn a detail row with label and value.
fn spawn_detail_row(parent: &mut ChildBuilder, label: &str, value: &str, color: Color) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(10.0),
            margin: UiRect::bottom(Val::Px(8.0)),
            ..default()
        })
        .with_children(|row| {
            // Label
            row.spawn((
                Text::new(label),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                Node {
                    width: Val::Px(100.0),
                    ..default()
                },
            ));

            // Value
            row.spawn((
                Text::new(value),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(color),
            ));
        });
}

/// System to update the inventory display when player inventory or selection changes.
fn update_inventory_display(
    mut commands: Commands,
    modal_root: Query<Entity, With<InventoryModalRoot>>,
    active_modal: Res<ActiveModal>,
    player: Res<Player>,
    mut selection: ResMut<InventorySelection>,
) {
    // Only update if the inventory modal is open
    if active_modal.modal != Some(ModalType::Inventory) {
        return;
    }

    // Only rebuild if player or selection changed
    if !player.is_changed() && !selection.is_changed() {
        return;
    }

    // Despawn and respawn the modal to update it
    if let Ok(entity) = modal_root.get_single() {
        commands.entity(entity).despawn_recursive();

        // Respawn with updated data
        spawn_inventory_modal(&mut commands, &player, &mut selection);
    }
}

/// Get the display color for an item quality.
pub fn get_quality_color(quality: &crate::item::enums::ItemQuality) -> Color {
    use crate::item::enums::ItemQuality;
    match quality {
        ItemQuality::Poor => Color::srgb(0.6, 0.6, 0.6),
        ItemQuality::Normal => Color::srgb(1.0, 1.0, 1.0),
        ItemQuality::Improved => Color::srgb(0.3, 1.0, 0.3),
        ItemQuality::WellForged => Color::srgb(0.3, 0.5, 1.0),
        ItemQuality::Masterworked => Color::srgb(0.8, 0.3, 1.0),
        ItemQuality::Mythic => Color::srgb(1.0, 0.5, 0.0),
    }
}

/// Format an item type for display.
fn format_item_type(item_type: &ItemType) -> String {
    match item_type {
        ItemType::Equipment(eq) => format!("Equipment ({:?})", eq),
        ItemType::Material(mat) => format!("Material ({:?})", mat),
        ItemType::Consumable(con) => format!("Consumable ({:?})", con),
        ItemType::QuestItem => "Quest Item".to_string(),
    }
}
