//! Fight modal rendering.

use bevy::prelude::*;

use crate::assets::{FightBannerSlice, GameSprites, SpriteSheetKey};
use crate::mob::Health;
use crate::player::PlayerName;
use crate::stats::{HasStats, StatSheet};
use crate::ui::screens::health_bar::{HealthBarValues, SpriteHealthBar};
use crate::ui::widgets::spawn_three_slice_banner;

use super::super::modal::spawn_modal_overlay;
use crate::ui::PlayerAttackTimer;

use super::state::{
    FightModalButton, FightModalButtonSelection, FightModalCancelButton, FightModalMob,
    FightModalMobHealthBar, FightModalMobSprite, FightModalOkButton, FightModalPlayerHealthBar,
    FightModalPlayerSprite, FightModalRoot,
};

const SPRITE_SIZE: f32 = 128.0;
const BANNER_WIDTH: f32 = 160.0;
const CONTAINER_WIDTH: f32 = 400.0;
const CONTAINER_HEIGHT: f32 = 270.0; // Increased to accommodate health bars
const BUTTON_SIZE: (f32, f32) = (27.0, 19.5);
const HEALTH_BAR_SIZE: (f32, f32) = (120.0, 16.0);

/// System to spawn the fight modal UI.
pub fn do_spawn_fight_modal(
    mut commands: Commands,
    mob_res: Res<FightModalMob>,
    player_name: Res<PlayerName>,
    stats: Res<StatSheet>,
    game_sprites: Res<GameSprites>,
    mob_query: Query<&Health>,
) {
    // Get mob health from entity component
    let mob_health = mob_query.get(mob_res.entity).ok();
    let (mob_current_hp, mob_max_hp) = mob_health
        .map(|h| (h.current, h.max))
        .unwrap_or((0, 1)); // Fallback if entity not found

    let overlay = spawn_modal_overlay(&mut commands);

    commands
        .entity(overlay)
        .insert(FightModalRoot)
        .with_children(|parent| {
            // Modal container
            parent
                .spawn((
                    Node {
                        width: Val::Px(CONTAINER_WIDTH),
                        height: Val::Px(CONTAINER_HEIGHT),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::FlexStart,
                        padding: UiRect::axes(Val::Px(10.0), Val::Px(15.0)),
                        border: UiRect::all(Val::Px(3.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.12, 0.1)),
                    BorderColor(Color::srgb(0.6, 0.5, 0.3)),
                ))
                .with_children(|container| {
                    // Player column (banner + sprite)
                    container
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::FlexStart,
                            row_gap: Val::Px(4.0),
                            ..default()
                        })
                        .with_children(|column| {
                            // Banner above player
                            spawn_three_slice_banner::<FightBannerSlice>(
                                column,
                                &game_sprites,
                                BANNER_WIDTH,
                                Some(player_name.0),
                            );

                            // Health bar
                            column.spawn((
                                FightModalPlayerHealthBar,
                                SpriteHealthBar,
                                HealthBarValues {
                                    current: stats.hp(),
                                    max: stats.max_hp(),
                                },
                                Node {
                                    width: Val::Px(HEALTH_BAR_SIZE.0),
                                    height: Val::Px(HEALTH_BAR_SIZE.1),
                                    ..default()
                                },
                            ));

                            // Player sprite (facing right - default orientation)
                            column.spawn((
                                FightModalPlayerSprite,
                                PlayerAttackTimer(Timer::from_seconds(0.54, TimerMode::Once)),
                                Node {
                                    width: Val::Px(SPRITE_SIZE),
                                    height: Val::Px(SPRITE_SIZE),
                                    ..default()
                                },
                            ));

                            // Button row below player sprite
                            column
                                .spawn(Node {
                                    flex_direction: FlexDirection::Row,
                                    column_gap: Val::Px(2.0),
                                    ..default()
                                })
                                .with_children(|row| {
                                    // OK button (left) - starts selected
                                    if let Some(sheet) = game_sprites.get(SpriteSheetKey::OkButtonSelected) {
                                        if let Some(bundle) = sheet.image_bundle("ok_button_selected", BUTTON_SIZE.0, BUTTON_SIZE.1) {
                                            row.spawn((FightModalOkButton, bundle));
                                        }
                                    }
                                    // Cancel button (right) - starts unselected
                                    if let Some(sheet) = game_sprites.get(SpriteSheetKey::CancelButton) {
                                        if let Some(bundle) = sheet.image_bundle("cancel_button", BUTTON_SIZE.0, BUTTON_SIZE.1) {
                                            row.spawn((FightModalCancelButton, bundle));
                                        }
                                    }
                                });
                        });

                    // Mob column (banner + sprite)
                    container
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::FlexStart,
                            row_gap: Val::Px(4.0),
                            ..default()
                        })
                        .with_children(|column| {
                            // Banner above mob
                            spawn_three_slice_banner::<FightBannerSlice>(
                                column,
                                &game_sprites,
                                BANNER_WIDTH,
                                Some(&mob_res.mob_id.spec().name),
                            );

                            // Health bar (from entity component)
                            column.spawn((
                                FightModalMobHealthBar,
                                SpriteHealthBar,
                                HealthBarValues {
                                    current: mob_current_hp,
                                    max: mob_max_hp,
                                },
                                Node {
                                    width: Val::Px(HEALTH_BAR_SIZE.0),
                                    height: Val::Px(HEALTH_BAR_SIZE.1),
                                    ..default()
                                },
                            ));

                            // Mob sprite (flipped to face left)
                            column.spawn((
                                FightModalMobSprite {
                                    mob_id: mob_res.mob_id,
                                },
                                Node {
                                    width: Val::Px(SPRITE_SIZE),
                                    height: Val::Px(SPRITE_SIZE),
                                    ..default()
                                },
                            ));
                        });
                });
        });
}

/// System to update button sprites based on selection state.
pub fn update_button_sprites(
    selection: Res<FightModalButtonSelection>,
    game_sprites: Res<GameSprites>,
    mut ok_query: Query<&mut ImageNode, (With<FightModalOkButton>, Without<FightModalCancelButton>)>,
    mut cancel_query: Query<&mut ImageNode, (With<FightModalCancelButton>, Without<FightModalOkButton>)>,
) {
    if !selection.is_changed() {
        return;
    }

    let ok_selected = selection.selected == FightModalButton::Ok;

    // Update OK button
    if let Ok(mut image) = ok_query.get_single_mut() {
        let key = if ok_selected {
            SpriteSheetKey::OkButtonSelected
        } else {
            SpriteSheetKey::OkButton
        };
        let sprite_name = if ok_selected { "ok_button_selected" } else { "ok_button" };
        if let Some(sheet) = game_sprites.get(key) {
            if let Some(node) = sheet.image_node(sprite_name) {
                *image = node;
            }
        }
    }

    // Update Cancel button
    if let Ok(mut image) = cancel_query.get_single_mut() {
        let key = if ok_selected {
            SpriteSheetKey::CancelButton
        } else {
            SpriteSheetKey::CancelButtonSelected
        };
        let sprite_name = if ok_selected { "cancel_button" } else { "cancel_button_selected" };
        if let Some(sheet) = game_sprites.get(key) {
            if let Some(node) = sheet.image_node(sprite_name) {
                *image = node;
            }
        }
    }
}

/// System to update mob health bar values from mob entity's Health component.
pub fn update_mob_health_bar(
    fight_mob: Res<FightModalMob>,
    mut bar_query: Query<&mut HealthBarValues, With<FightModalMobHealthBar>>,
    mob_query: Query<&Health>,
) {
    let Ok(mut values) = bar_query.get_single_mut() else {
        return;
    };
    let Ok(health) = mob_query.get(fight_mob.entity) else {
        return;
    };
    values.current = health.current;
    values.max = health.max;
}

/// System to update player health bar values from StatSheet.
pub fn update_player_health_bar(
    stats: Res<StatSheet>,
    mut bar_query: Query<&mut HealthBarValues, With<FightModalPlayerHealthBar>>,
) {
    let Ok(mut values) = bar_query.get_single_mut() else {
        return;
    };
    values.current = stats.hp();
    values.max = stats.max_hp();
}
