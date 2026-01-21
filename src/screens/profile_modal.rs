use bevy::prelude::*;

use crate::combat::{DealsDamage, HasGold};
use crate::entities::progression::HasProgression;
use crate::entities::Progression;
use crate::input::GameAction;
use crate::inventory::{EquipmentSlot, HasInventory, Inventory};
use crate::player::{Player, PlayerGold, PlayerName};
use crate::screens::modal::{
    close_modal, create_modal_container, create_modal_instruction, create_modal_title,
    spawn_modal_overlay, toggle_modal, ActiveModal, ModalAction, ModalType,
};
use crate::stats::{HasStats, StatSheet, StatType};
use crate::ui::widgets::StatRow;

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
    name: Res<PlayerName>,
    gold: Res<PlayerGold>,
    progression: Res<Progression>,
    inventory: Res<Inventory>,
    stats: Res<StatSheet>,
    existing_modal: Query<Entity, With<ProfileModalRoot>>,
) {
    for action in action_reader.read() {
        if *action == GameAction::OpenProfile {
            if let Some(ModalAction::Open) = toggle_modal(
                &mut commands,
                &mut active_modal,
                &existing_modal,
                ModalType::Profile,
            ) {
                let player = Player::from_resources(&name, &gold, &progression, &inventory, &stats);
                spawn_profile_modal(&mut commands, &player);
            }
        }
    }
}

/// System to handle closing the profile modal with Escape.
fn handle_profile_modal_close(
    mut commands: Commands,
    mut action_reader: EventReader<GameAction>,
    mut active_modal: ResMut<ActiveModal>,
    modal_query: Query<Entity, With<ProfileModalRoot>>,
) {
    for action in action_reader.read() {
        if *action == GameAction::CloseModal {
            close_modal(
                &mut commands,
                &mut active_modal,
                &modal_query,
                ModalType::Profile,
            );
        }
    }
}

/// Spawn the profile modal UI showing player stats and equipped items.
fn spawn_profile_modal(commands: &mut Commands, player: &Player) {
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
fn spawn_stats_column(parent: &mut ChildBuilder, player: &Player) {
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
            stats.spawn(
                StatRow::new("Name:", player.name)
                    .label_width(140.0)
                    .font_size(22.0)
                    .label_color(Color::srgb(0.8, 0.8, 0.8))
                    .value_color(Color::srgb(0.9, 0.9, 0.9)),
            );
            stats.spawn(
                StatRow::new("Level:", format!("{}", player.level()))
                    .label_width(140.0)
                    .font_size(22.0)
                    .label_color(Color::srgb(0.8, 0.8, 0.8))
                    .value_color(Color::srgb(0.6, 1.0, 0.6)),
            );

            // HP
            stats.spawn(
                StatRow::new("HP:", format!("{} / {}", player.hp(), player.max_hp()))
                    .label_width(140.0)
                    .font_size(22.0)
                    .label_color(Color::srgb(0.8, 0.8, 0.8))
                    .value_color(Color::srgb(0.95, 0.3, 0.3)),
            );

            // Gold
            stats.spawn(
                StatRow::new("Gold:", format!("{}", player.gold()))
                    .label_width(140.0)
                    .font_size(22.0)
                    .label_color(Color::srgb(0.8, 0.8, 0.8))
                    .value_color(Color::srgb(1.0, 0.84, 0.0)),
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

            let attack_row = StatRow::new("Attack:", format!("{}", attack))
                .label_width(140.0)
                .font_size(22.0)
                .label_color(Color::srgb(0.8, 0.8, 0.8))
                .value_color(Color::srgb(1.0, 0.4, 0.2));
            if attack_bonus > 0 {
                stats.spawn(
                    attack_row.with_bonus(format!("(+{})", attack_bonus), Color::srgb(0.4, 1.0, 0.4)),
                );
            } else {
                stats.spawn(attack_row);
            }

            let defense_row = StatRow::new("Defense:", format!("{}", defense))
                .label_width(140.0)
                .font_size(22.0)
                .label_color(Color::srgb(0.8, 0.8, 0.8))
                .value_color(Color::srgb(0.4, 0.6, 1.0));
            if defense_bonus > 0 {
                stats.spawn(
                    defense_row.with_bonus(format!("(+{})", defense_bonus), Color::srgb(0.4, 1.0, 0.4)),
                );
            } else {
                stats.spawn(defense_row);
            }

            // Separator
            stats.spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Px(2.0),
                margin: UiRect::vertical(Val::Px(10.0)),
                ..default()
            });

            // Additional stats
            stats.spawn(
                StatRow::new("Gold Find:", format!("+{}%", player.effective_goldfind()))
                    .label_width(140.0)
                    .font_size(22.0)
                    .label_color(Color::srgb(0.8, 0.8, 0.8))
                    .value_color(Color::srgb(1.0, 0.84, 0.0)),
            );
            stats.spawn(
                StatRow::new("Magic Find:", format!("+{}%", player.effective_magicfind()))
                    .label_width(140.0)
                    .font_size(22.0)
                    .label_color(Color::srgb(0.8, 0.8, 0.8))
                    .value_color(Color::srgb(0.7, 0.4, 1.0)),
            );
            stats.spawn(
                StatRow::new("Mining:", format!("{}", player.effective_mining()))
                    .label_width(140.0)
                    .font_size(22.0)
                    .label_color(Color::srgb(0.8, 0.8, 0.8))
                    .value_color(Color::srgb(0.7, 0.7, 0.7)),
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

            stats.spawn(
                StatRow::new("XP:", format!("{} / {} ({}%)", xp, xp_needed, xp_percent))
                    .label_width(140.0)
                    .font_size(22.0)
                    .label_color(Color::srgb(0.8, 0.8, 0.8))
                    .value_color(Color::srgb(0.8, 0.5, 1.0)),
            );

            // XP Bar
            spawn_xp_bar(stats, xp, xp_needed);
        });
}

/// Spawn the right column showing equipped items.
fn spawn_equipment_column(parent: &mut ChildBuilder, player: &Player) {
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
                let (item_name, color) = if let Some(inv_item) = player.inventory().equipment().get(slot) {
                    (inv_item.item.name.clone(), inv_item.item.quality.color())
                } else {
                    ("(Empty)".to_string(), Color::srgb(0.5, 0.5, 0.5))
                };
                equipment.spawn(
                    StatRow::new(*label, item_name)
                        .label_width(100.0)
                        .font_size(20.0)
                        .label_color(Color::srgb(0.7, 0.7, 0.7))
                        .value_color(color),
                );
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
