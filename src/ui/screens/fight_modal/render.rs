//! Fight modal rendering.

use bevy::prelude::*;

use crate::assets::{FightBannerSlice, GameSprites};
use crate::player::PlayerName;
use crate::ui::widgets::spawn_three_slice_banner;

use super::super::modal::{spawn_modal_overlay, ActiveModal, ModalType};
use super::state::{FightModalMob, FightModalMobSprite, FightModalPlayerSprite, FightModalRoot, SpawnFightModal};

const SPRITE_SIZE: f32 = 128.0;
const BANNER_WIDTH: f32 = 160.0;
const CONTAINER_WIDTH: f32 = 400.0;
const CONTAINER_HEIGHT: f32 = 250.0; // Increased to accommodate banners

/// System to spawn the fight modal UI.
pub fn spawn_fight_modal(
    mut commands: Commands,
    mob_res: Res<FightModalMob>,
    player_name: Res<PlayerName>,
    mut active_modal: ResMut<ActiveModal>,
    game_sprites: Res<GameSprites>,
) {
    commands.remove_resource::<SpawnFightModal>();
    active_modal.modal = Some(ModalType::FightModal);

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

                            // Player sprite (facing right - default orientation)
                            column.spawn((
                                FightModalPlayerSprite,
                                Node {
                                    width: Val::Px(SPRITE_SIZE),
                                    height: Val::Px(SPRITE_SIZE),
                                    ..default()
                                },
                            ));
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
