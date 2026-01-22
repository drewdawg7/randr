//! Victory modal rendering.

use bevy::prelude::*;

use crate::ui::screens::modal::{ActiveModal, ModalType};
use crate::ui::widgets::Column;
use crate::ui::{Modal, SpawnModalExt};

use super::state::{SpawnVictoryModal, VictoryModalData, VictoryModalMobSprite, VictoryModalRoot};

const SPRITE_SIZE: f32 = 128.0;
const MODAL_WIDTH: f32 = 300.0;
const MOB_NAME_FONT_SIZE: f32 = 24.0;
const STAT_FONT_SIZE: f32 = 22.0;
const LOOT_FONT_SIZE: f32 = 20.0;

const GOLD_COLOR: Color = Color::srgb(1.0, 0.84, 0.0);
const XP_COLOR: Color = Color::srgb(0.6, 0.8, 1.0);

/// System to spawn the victory modal UI.
pub fn spawn_victory_modal(
    mut commands: Commands,
    data: Res<VictoryModalData>,
    mut active_modal: ResMut<ActiveModal>,
) {
    commands.remove_resource::<SpawnVictoryModal>();
    active_modal.modal = Some(ModalType::VictoryModal);

    let mob_name = data.mob_name.clone();
    let mob_id = data.mob_id;
    let gold_gained = data.gold_gained;
    let xp_gained = data.xp_gained;
    let loot_drops: Vec<(String, i32)> = data
        .loot_drops
        .iter()
        .map(|drop| (drop.item.name.clone(), drop.quantity))
        .collect();

    commands.spawn_modal(
        Modal::new()
            .title("Victory!")
            .size(MODAL_WIDTH, 0.0)
            .with_root_marker(|e| {
                e.insert(VictoryModalRoot);
            })
            .content(move |c| {
                c.spawn(Column::new().gap(8.0).align_center())
                    .with_children(|col| {
                        // Mob sprite
                        col.spawn((
                            VictoryModalMobSprite { mob_id },
                            Node {
                                width: Val::Px(SPRITE_SIZE),
                                height: Val::Px(SPRITE_SIZE),
                                ..default()
                            },
                        ));

                        // Mob name
                        col.spawn((
                            Text::new(&mob_name),
                            TextFont {
                                font_size: MOB_NAME_FONT_SIZE,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));

                        // Gold gained
                        col.spawn((
                            Text::new(format!("+{gold_gained} Gold")),
                            TextFont {
                                font_size: STAT_FONT_SIZE,
                                ..default()
                            },
                            TextColor(GOLD_COLOR),
                        ));

                        // XP gained
                        col.spawn((
                            Text::new(format!("+{xp_gained} XP")),
                            TextFont {
                                font_size: STAT_FONT_SIZE,
                                ..default()
                            },
                            TextColor(XP_COLOR),
                        ));

                        // Loot drops (if any)
                        for (item_name, quantity) in &loot_drops {
                            let text = if *quantity > 1 {
                                format!("{item_name} x{quantity}")
                            } else {
                                item_name.clone()
                            };
                            col.spawn((
                                Text::new(text),
                                TextFont {
                                    font_size: LOOT_FONT_SIZE,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        }
                    });
            }),
    );
}
