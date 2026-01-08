use bevy::prelude::*;

use crate::combat::{DealsDamage, HasGold};
use crate::entities::progression::HasProgression;
use crate::game::PlayerResource;
use crate::input::GameAction;
use crate::inventory::{EquipmentSlot, HasInventory};
use crate::screens::modal::{
    create_modal_container, create_modal_instruction, create_modal_title, spawn_modal_overlay,
    ActiveModal, ModalType,
};
use crate::stats::{HasStats, StatType};

/// Plugin that manages the profile modal system.
pub struct ProfileModalPlugin;

impl Plugin for ProfileModalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_profile_modal_toggle, handle_profile_modal_close),
        );
    }
}

/// Component marker for the profile modal UI.
#[derive(Component)]
pub struct ProfileModalRoot;

/// System to handle opening the profile modal with 'p' key.
fn handle_profile_modal_toggle(
    mut commands: Commands,
    mut action_reader: EventReader<GameAction>,
    mut active_modal: ResMut<ActiveModal>,
    player: Res<PlayerResource>,
    existing_modal: Query<Entity, With<ProfileModalRoot>>,
) {
    for action in action_reader.read() {
        if *action == GameAction::OpenProfile {
            // Close existing modal if open
            if let Ok(entity) = existing_modal.get_single() {
                commands.entity(entity).despawn_recursive();
                active_modal.modal = None;
            } else {
                // Open new modal
                spawn_profile_modal(&mut commands, &player);
                active_modal.modal = Some(ModalType::Profile);
            }
        }
    }
}

/// System to handle closing the profile modal with Escape or 'p' key.
fn handle_profile_modal_close(
    mut commands: Commands,
    mut action_reader: EventReader<GameAction>,
    mut active_modal: ResMut<ActiveModal>,
    modal_query: Query<Entity, With<ProfileModalRoot>>,
) {
    for action in action_reader.read() {
        if matches!(action, GameAction::CloseModal | GameAction::OpenProfile) {
            if active_modal.modal == Some(ModalType::Profile) {
                if let Ok(entity) = modal_query.get_single() {
                    commands.entity(entity).despawn_recursive();
                    active_modal.modal = None;
                }
            }
        }
    }
}

/// Spawn the profile modal UI showing player stats and equipped items.
fn spawn_profile_modal(commands: &mut Commands, player: &PlayerResource) {
    let overlay = spawn_modal_overlay(commands);

    commands
        .entity(overlay)
        .insert(ProfileModalRoot)
        .with_children(|parent| {
            // Modal content container
            parent
                .spawn((
                    create_modal_container(),
                    BackgroundColor(Color::srgb(0.15, 0.12, 0.1)),
                    BorderColor(Color::srgb(0.6, 0.5, 0.3)),
                ))
                .with_children(|modal| {
                    // Title
                    modal.spawn(create_modal_title("Character Profile"));

                    // Two-column layout
                    modal
                        .spawn(Node {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(40.0),
                            ..default()
                        })
                        .with_children(|columns| {
                            // Left column: Stats
                            spawn_stats_column(columns, player);

                            // Right column: Equipment
                            spawn_equipment_column(columns, player);
                        });

                    // Instructions at bottom
                    modal.spawn(create_modal_instruction(
                        "Press [P] or [Esc] to close",
                    ));
                });
        });
}

/// Spawn the left column showing player stats.
fn spawn_stats_column(parent: &mut ChildBuilder, player: &PlayerResource) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            width: Val::Percent(50.0),
            row_gap: Val::Px(12.0),
            ..default()
        })
        .with_children(|stats| {
            // Section header
            stats.spawn((
                Text::new("Character Stats"),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.8, 0.5)),
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
            ));

            // Name and Level
            spawn_stat_row(
                stats,
                "Name:",
                player.name,
                Color::srgb(0.9, 0.9, 0.9),
            );
            spawn_stat_row(
                stats,
                "Level:",
                &format!("{}", player.level()),
                Color::srgb(0.6, 1.0, 0.6),
            );

            // HP
            spawn_stat_row(
                stats,
                "HP:",
                &format!("{} / {}", player.hp(), player.max_hp()),
                Color::srgb(0.95, 0.3, 0.3),
            );

            // Gold
            spawn_stat_row(
                stats,
                "Gold:",
                &format!("{}", player.gold()),
                Color::srgb(1.0, 0.84, 0.0),
            );

            // Separator
            stats.spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Px(2.0),
                margin: UiRect::vertical(Val::Px(10.0)),
                ..default()
            });

            // Combat stats
            let attack = player.attack();
            let attack_bonus = player.equipment_attack_bonus();
            let defense = player.defense();
            let defense_bonus = player.inventory().sum_equipment_stats(StatType::Defense);

            if attack_bonus > 0 {
                spawn_stat_row_with_bonus(
                    stats,
                    "Attack:",
                    &format!("{}", attack),
                    &format!("(+{})", attack_bonus),
                    Color::srgb(1.0, 0.4, 0.2),
                    Color::srgb(0.4, 1.0, 0.4),
                );
            } else {
                spawn_stat_row(
                    stats,
                    "Attack:",
                    &format!("{}", attack),
                    Color::srgb(1.0, 0.4, 0.2),
                );
            }

            if defense_bonus > 0 {
                spawn_stat_row_with_bonus(
                    stats,
                    "Defense:",
                    &format!("{}", defense),
                    &format!("(+{})", defense_bonus),
                    Color::srgb(0.4, 0.6, 1.0),
                    Color::srgb(0.4, 1.0, 0.4),
                );
            } else {
                spawn_stat_row(
                    stats,
                    "Defense:",
                    &format!("{}", defense),
                    Color::srgb(0.4, 0.6, 1.0),
                );
            }

            // Separator
            stats.spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Px(2.0),
                margin: UiRect::vertical(Val::Px(10.0)),
                ..default()
            });

            // Additional stats
            spawn_stat_row(
                stats,
                "Gold Find:",
                &format!("+{}%", player.effective_goldfind()),
                Color::srgb(1.0, 0.84, 0.0),
            );
            spawn_stat_row(
                stats,
                "Magic Find:",
                &format!("+{}%", player.effective_magicfind()),
                Color::srgb(0.7, 0.4, 1.0),
            );
            spawn_stat_row(
                stats,
                "Mining:",
                &format!("{}", player.effective_mining()),
                Color::srgb(0.7, 0.7, 0.7),
            );

            // Separator
            stats.spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Px(2.0),
                margin: UiRect::vertical(Val::Px(10.0)),
                ..default()
            });

            // XP Progress
            let xp = player.prog.xp;
            let xp_needed = crate::entities::Progression::xp_to_next_level(player.level());
            let xp_percent = if xp_needed > 0 {
                (xp as f32 / xp_needed as f32 * 100.0) as i32
            } else {
                100
            };

            spawn_stat_row(
                stats,
                "XP:",
                &format!("{} / {} ({}%)", xp, xp_needed, xp_percent),
                Color::srgb(0.8, 0.5, 1.0),
            );

            // XP Bar
            spawn_xp_bar(stats, xp, xp_needed);
        });
}

/// Spawn the right column showing equipped items.
fn spawn_equipment_column(parent: &mut ChildBuilder, player: &PlayerResource) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            width: Val::Percent(50.0),
            row_gap: Val::Px(12.0),
            ..default()
        })
        .with_children(|equipment| {
            // Section header
            equipment.spawn((
                Text::new("Equipment"),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.8, 0.5)),
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
            ));

            // Display each equipment slot
            let slots = [
                (EquipmentSlot::Head, "Head:"),
                (EquipmentSlot::Chest, "Chest:"),
                (EquipmentSlot::Weapon, "Weapon:"),
                (EquipmentSlot::OffHand, "Off-Hand:"),
                (EquipmentSlot::Hands, "Hands:"),
                (EquipmentSlot::Feet, "Feet:"),
                (EquipmentSlot::Legs, "Legs:"),
                (EquipmentSlot::Ring, "Ring:"),
                (EquipmentSlot::Tool, "Tool:"),
            ];

            for (slot, label) in slots.iter() {
                if let Some(inv_item) = player.inventory().equipment().get(slot) {
                    let item = &inv_item.item;
                    spawn_equipment_row(
                        equipment,
                        label,
                        &item.name,
                        get_quality_color(&item.quality),
                    );
                } else {
                    spawn_equipment_row(
                        equipment,
                        label,
                        "(Empty)",
                        Color::srgb(0.5, 0.5, 0.5),
                    );
                }
            }

            // Separator
            equipment.spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Px(2.0),
                margin: UiRect::vertical(Val::Px(10.0)),
                ..default()
            });

            // Total slots used
            let total_items = player.inventory().items.len();
            let max_slots = player.inventory().max_slots();
            equipment.spawn((
                Text::new(format!("Inventory: {} / {} slots", total_items, max_slots)),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));
        });
}

/// Helper to spawn a stat row with label and value.
fn spawn_stat_row(parent: &mut ChildBuilder, label: &str, value: &str, color: Color) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(10.0),
            ..default()
        })
        .with_children(|row| {
            // Label
            row.spawn((
                Text::new(label),
                TextFont {
                    font_size: 22.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                Node {
                    width: Val::Px(140.0),
                    ..default()
                },
            ));

            // Value
            row.spawn((
                Text::new(value),
                TextFont {
                    font_size: 22.0,
                    ..default()
                },
                TextColor(color),
            ));
        });
}

/// Helper to spawn a stat row with bonus text.
fn spawn_stat_row_with_bonus(
    parent: &mut ChildBuilder,
    label: &str,
    value: &str,
    bonus: &str,
    color: Color,
    bonus_color: Color,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(10.0),
            ..default()
        })
        .with_children(|row| {
            // Label
            row.spawn((
                Text::new(label),
                TextFont {
                    font_size: 22.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                Node {
                    width: Val::Px(140.0),
                    ..default()
                },
            ));

            // Value
            row.spawn((
                Text::new(value),
                TextFont {
                    font_size: 22.0,
                    ..default()
                },
                TextColor(color),
            ));

            // Bonus
            row.spawn((
                Text::new(format!(" {}", bonus)),
                TextFont {
                    font_size: 22.0,
                    ..default()
                },
                TextColor(bonus_color),
            ));
        });
}

/// Helper to spawn an equipment row.
fn spawn_equipment_row(
    parent: &mut ChildBuilder,
    label: &str,
    item_name: &str,
    color: Color,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(10.0),
            ..default()
        })
        .with_children(|row| {
            // Slot label
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

            // Item name
            row.spawn((
                Text::new(item_name),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(color),
            ));
        });
}

/// Helper to spawn an XP progress bar.
fn spawn_xp_bar(parent: &mut ChildBuilder, current: i32, max: i32) {
    let filled_percent = if max > 0 {
        (current as f32 / max as f32 * 100.0).min(100.0)
    } else {
        100.0
    };

    parent
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Px(24.0),
            border: UiRect::all(Val::Px(2.0)),
            margin: UiRect::top(Val::Px(5.0)),
            ..default()
        })
        .with_children(|bar_container| {
            // Filled portion
            bar_container.spawn((
                Node {
                    width: Val::Percent(filled_percent),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.6, 0.4, 0.8)),
            ));
        });
}

/// Get the display color for an item quality.
fn get_quality_color(quality: &crate::item::enums::ItemQuality) -> Color {
    use crate::item::enums::ItemQuality;
    match quality {
        ItemQuality::Poor => Color::srgb(0.6, 0.6, 0.6),
        ItemQuality::Normal => Color::srgb(1.0, 1.0, 1.0),
        ItemQuality::Improved => Color::srgb(0.3, 1.0, 0.3),
        ItemQuality::WellForged => Color::srgb(0.4, 0.6, 1.0),
        ItemQuality::Masterworked => Color::srgb(0.8, 0.4, 1.0),
        ItemQuality::Mythic => Color::srgb(1.0, 0.6, 0.0),
    }
}
