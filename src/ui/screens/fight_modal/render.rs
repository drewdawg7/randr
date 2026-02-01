use bevy::prelude::*;

use crate::assets::{FightBannerSlice, GameSprites, SpriteSheetKey};
use crate::mob::{Health, MobMarker};
use crate::player::PlayerName;
use crate::stats::{HasStats, StatSheet};
use crate::ui::screens::health_bar::SpriteHealthBarBundle;
use crate::ui::widgets::spawn_three_slice_banner;
use crate::ui::PlayerAttackTimer;
use crate::ui::{Modal, SpawnModalExt};

use super::state::{
    FightModalButton, FightModalButtonSelection, FightModalCancelButton, FightModalMob,
    FightModalMobHealthBar, FightModalMobSprite, FightModalOkButton, FightModalPlayerHealthBar,
    FightModalPlayerSprite, FightModalRoot,
};

const SPRITE_SIZE: f32 = 128.0;
const BANNER_WIDTH: f32 = 160.0;
const CONTAINER_WIDTH: f32 = 400.0;
const CONTAINER_HEIGHT: f32 = 270.0;
const BUTTON_SIZE: (f32, f32) = (27.0, 19.5);
const HEALTH_BAR_SIZE: (f32, f32) = (120.0, 16.0);

pub fn do_spawn_fight_modal(
    mut commands: Commands,
    mob_res: Res<FightModalMob>,
    player_name: Res<PlayerName>,
    stats: Res<StatSheet>,
    game_sprites: Res<GameSprites>,
    mob_query: Query<&Health>,
) {
    let mob_health = mob_query.get(mob_res.entity).ok();
    let (mob_current_hp, mob_max_hp) = mob_health.map(|h| (h.current, h.max)).unwrap_or((0, 1));

    let player_name_str = player_name.0.clone();
    let player_hp = stats.hp();
    let player_max_hp = stats.max_hp();
    let mob_id = mob_res.mob_id;
    let mob_name = mob_res.mob_id.spec().name.clone();
    let game_sprites = (*game_sprites).clone();

    commands.spawn_modal(
        Modal::builder()
            .size((CONTAINER_WIDTH, CONTAINER_HEIGHT))
            .padding(0.0)
            .root_marker(Box::new(|e| {
                e.insert(FightModalRoot);
            }))
            .content(Box::new(move |c| {
                c.spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::FlexStart,
                    padding: UiRect::axes(Val::Px(10.0), Val::Px(15.0)),
                    ..default()
                })
                .with_children(|container| {
                    spawn_player_column(
                        container,
                        &game_sprites,
                        &player_name_str,
                        player_hp,
                        player_max_hp,
                    );
                    spawn_mob_column(
                        container,
                        &game_sprites,
                        &mob_name,
                        mob_id,
                        mob_current_hp,
                        mob_max_hp,
                    );
                });
            }))
            .build(),
    );
}

fn spawn_player_column(
    parent: &mut ChildSpawnerCommands,
    game_sprites: &GameSprites,
    player_name: &str,
    hp: i32,
    max_hp: i32,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::FlexStart,
            row_gap: Val::Px(4.0),
            ..default()
        })
        .with_children(|column| {
            spawn_three_slice_banner::<FightBannerSlice>(
                column,
                game_sprites,
                BANNER_WIDTH,
                Some(player_name),
            );

            column.spawn((
                FightModalPlayerHealthBar,
                SpriteHealthBarBundle::new(hp, max_hp, HEALTH_BAR_SIZE.0, HEALTH_BAR_SIZE.1),
            ));

            column.spawn((
                FightModalPlayerSprite,
                PlayerAttackTimer(Timer::from_seconds(0.54, TimerMode::Once)),
                Node {
                    width: Val::Px(SPRITE_SIZE),
                    height: Val::Px(SPRITE_SIZE),
                    ..default()
                },
            ));

            column
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(2.0),
                    ..default()
                })
                .with_children(|row| {
                    if let Some(sheet) = game_sprites.get(SpriteSheetKey::OkButtonSelected) {
                        if let Some(bundle) =
                            sheet.image_bundle("ok_button_selected", BUTTON_SIZE.0, BUTTON_SIZE.1)
                        {
                            row.spawn((FightModalOkButton, bundle));
                        }
                    }
                    if let Some(sheet) = game_sprites.get(SpriteSheetKey::CancelButton) {
                        if let Some(bundle) =
                            sheet.image_bundle("cancel_button", BUTTON_SIZE.0, BUTTON_SIZE.1)
                        {
                            row.spawn((FightModalCancelButton, bundle));
                        }
                    }
                });
        });
}

fn spawn_mob_column(
    parent: &mut ChildSpawnerCommands,
    game_sprites: &GameSprites,
    mob_name: &str,
    mob_id: crate::mob::MobId,
    current_hp: i32,
    max_hp: i32,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::FlexStart,
            row_gap: Val::Px(4.0),
            ..default()
        })
        .with_children(|column| {
            spawn_three_slice_banner::<FightBannerSlice>(
                column,
                game_sprites,
                BANNER_WIDTH,
                Some(mob_name),
            );

            column.spawn((
                FightModalMobHealthBar,
                SpriteHealthBarBundle::new(current_hp, max_hp, HEALTH_BAR_SIZE.0, HEALTH_BAR_SIZE.1),
            ));

            column.spawn((
                FightModalMobSprite { mob_id },
                Node {
                    width: Val::Px(SPRITE_SIZE),
                    height: Val::Px(SPRITE_SIZE),
                    ..default()
                },
            ));
        });
}

pub fn update_button_sprites(
    selection: Res<FightModalButtonSelection>,
    game_sprites: Res<GameSprites>,
    mut ok_query: Query<
        &mut ImageNode,
        (With<FightModalOkButton>, Without<FightModalCancelButton>),
    >,
    mut cancel_query: Query<
        &mut ImageNode,
        (With<FightModalCancelButton>, Without<FightModalOkButton>),
    >,
) {
    if !selection.is_changed() {
        return;
    }

    let ok_selected = selection.selected == FightModalButton::Ok;

    if let Ok(mut image) = ok_query.single_mut() {
        let key = if ok_selected {
            SpriteSheetKey::OkButtonSelected
        } else {
            SpriteSheetKey::OkButton
        };
        let sprite_name = if ok_selected {
            "ok_button_selected"
        } else {
            "ok_button"
        };
        if let Some(sheet) = game_sprites.get(key) {
            if let Some(node) = sheet.image_node(sprite_name) {
                *image = node;
            }
        }
    }

    if let Ok(mut image) = cancel_query.single_mut() {
        let key = if ok_selected {
            SpriteSheetKey::CancelButton
        } else {
            SpriteSheetKey::CancelButtonSelected
        };
        let sprite_name = if ok_selected {
            "cancel_button"
        } else {
            "cancel_button_selected"
        };
        if let Some(sheet) = game_sprites.get(key) {
            if let Some(node) = sheet.image_node(sprite_name) {
                *image = node;
            }
        }
    }
}

pub fn update_mob_health_bar(
    fight_mob: Res<FightModalMob>,
    mut bar_query: Query<&mut Health, (With<FightModalMobHealthBar>, Without<MobMarker>)>,
    mob_query: Query<&Health, (With<MobMarker>, Changed<Health>)>,
) {
    let Ok(mut bar_health) = bar_query.single_mut() else {
        return;
    };
    let Ok(mob_health) = mob_query.get(fight_mob.entity) else {
        return;
    };
    bar_health.current = mob_health.current;
    bar_health.max = mob_health.max;
}

pub fn update_player_health_bar(
    stats: Res<StatSheet>,
    mut bar_query: Query<&mut Health, With<FightModalPlayerHealthBar>>,
) {
    if !stats.is_changed() {
        return;
    }
    let Ok(mut health) = bar_query.single_mut() else {
        return;
    };
    health.current = stats.hp();
    health.max = stats.max_hp();
}
