use bevy::prelude::*;
use bevy::ui::widget::NodeImageMode;
use rand::seq::SliceRandom;

use crate::combat::{ActiveCombatResource, CombatPhaseState};
use crate::game::PlayerName;
use crate::assets::{GameAssets, GameSprites, SpriteSheetKey};
use crate::ui::{
    update_health_bar, HealthBarBundle, HealthBarNameBundle, HealthBarText, HealthBarTextBundle,
    SpriteHealthBar, SpriteHealthBarBundle,
};
use crate::stats::{HasStats, StatSheet};
use crate::ui::{nav_selection_text, MenuIndex};

use super::components::{
    ActionMenuItem, EnemyHealthBar, EnemyNameLabel, FightScreenRoot, NeedsFightBackground,
    NeedsMobSprite, PlayerHealthBar, PostCombatMenuItem, PostCombatOverlay, CombatResultText,
    RewardsText,
};
use super::state::FightScreenState;

/// Resource holding the selected fight background for the current fight.
#[derive(Resource, Default)]
pub struct SelectedFightBackground(pub Option<Handle<Image>>);

pub fn spawn_fight_screen(
    mut commands: Commands,
    name: Res<PlayerName>,
    stats: Res<StatSheet>,
    combat_res: Res<ActiveCombatResource>,
    game_assets: Res<GameAssets>,
    mut selected_bg: ResMut<SelectedFightBackground>,
) {
    // Select a random background
    selected_bg.0 = game_assets
        .sprites
        .fight_backgrounds
        .choose(&mut rand::thread_rng())
        .cloned();
    let (player_health, player_max_health, enemy_name, enemy_health, enemy_max_health) =
        if let Some(combat) = combat_res.get() {
            let enemy_info = combat.enemy_info();
            (
                stats.hp(),
                stats.max_hp(),
                enemy_info.name.clone(),
                enemy_info.health,
                enemy_info.max_health,
            )
        } else {
            (stats.hp(), stats.max_hp(), "Unknown".to_string(), 1, 1)
        };

    commands
        .spawn((
            FightScreenRoot,
            NeedsFightBackground,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::vertical(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
        ))
        .with_children(|parent| {
            spawn_combatants_section(parent, name.0, player_health, player_max_health, &enemy_name, enemy_health, enemy_max_health);

            // Action menu popup in bottom right
            if let Some(popup) = &game_assets.sprites.fight_popup {
                parent
                    .spawn((
                        ImageNode::new(popup.clone()).with_mode(NodeImageMode::Sliced(TextureSlicer {
                            border: BorderRect::square(8.0),
                            ..default()
                        })),
                        Node {
                            position_type: PositionType::Absolute,
                            bottom: Val::Px(20.0),
                            right: Val::Px(20.0),
                            width: Val::Px(240.0),
                            height: Val::Px(160.0),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::FlexStart,
                            justify_content: JustifyContent::FlexStart,
                            row_gap: Val::Px(8.0),
                            padding: UiRect::left(Val::Px(16.0)),
                            ..default()
                        },
                    ))
                    .with_children(|popup_parent| {
                        spawn_action_item(popup_parent, 0, "Attack", true);
                        spawn_action_item(popup_parent, 1, "Run", false);
                    });
            }
        });
}

fn spawn_combatants_section(
    parent: &mut ChildBuilder,
    player_name: &str,
    player_health: i32,
    player_max_health: i32,
    enemy_name: &str,
    enemy_health: i32,
    enemy_max_health: i32,
) {
    parent
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_grow: 1.0,
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        })
        .with_children(|combatants| {
            spawn_player_side(combatants, player_name, player_health, player_max_health);
            spawn_enemy_side(combatants, enemy_name, enemy_health, enemy_max_health);
        });
}

fn spawn_player_side(
    parent: &mut ChildBuilder,
    player_name: &str,
    health: i32,
    max_health: i32,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::FlexStart,
            ..default()
        })
        .with_children(|player_side| {
            player_side
                .spawn((
                    PlayerHealthBar,
                    HealthBarBundle::new(AlignItems::FlexStart),
                    BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
                ))
                .insert(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::FlexStart,
                    row_gap: Val::Px(5.0),
                    padding: UiRect::all(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|bar| {
                    bar.spawn(HealthBarNameBundle::new(player_name));
                    bar.spawn(SpriteHealthBarBundle::new(AlignSelf::FlexStart));
                    bar.spawn(HealthBarTextBundle::new(health, max_health));
                });
        });
}

fn spawn_enemy_side(
    parent: &mut ChildBuilder,
    enemy_name: &str,
    health: i32,
    max_health: i32,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::FlexEnd,
            height: Val::Percent(100.0),
            ..default()
        })
        .with_children(|enemy_side| {
            enemy_side
                .spawn((
                    EnemyHealthBar,
                    HealthBarBundle::new(AlignItems::FlexEnd),
                    BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
                ))
                .insert(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::FlexEnd,
                    row_gap: Val::Px(5.0),
                    padding: UiRect::all(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|bar| {
                    bar.spawn((EnemyNameLabel, HealthBarNameBundle::new(enemy_name)));
                    bar.spawn(SpriteHealthBarBundle::new(AlignSelf::FlexEnd));
                    bar.spawn(HealthBarTextBundle::new(health, max_health));
                });

            // Wrapper to vertically center the mob sprite container
            enemy_side
                .spawn(Node {
                    flex_grow: 1.0,
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|wrapper| {
                    // Mob sprite container with translucent background
                    wrapper
                        .spawn((
                            Node {
                                width: Val::Px(192.0),
                                height: Val::Px(192.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                padding: UiRect::all(Val::Px(8.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
                        ))
                        .with_children(|container| {
                            container.spawn((
                                NeedsMobSprite,
                                Node {
                                    width: Val::Px(160.0),
                                    height: Val::Px(160.0),
                                    ..default()
                                },
                            ));
                        });
                });
        });
}

fn spawn_action_item(parent: &mut ChildBuilder, index: usize, label: &str, selected: bool) {
    let suffix = if selected { " <" } else { "" };
    let color = if selected {
        Color::srgb(0.15, 0.1, 0.05)
    } else {
        Color::srgb(0.4, 0.35, 0.3)
    };
    parent.spawn((
        ActionMenuItem,
        MenuIndex(index),
        Text::new(format!("{}{}", label, suffix)),
        TextFont { font_size: 18.0, ..default() },
        TextColor(color),
    ));
}

pub fn spawn_post_combat_overlay(
    mut commands: Commands,
    combat_res: Res<ActiveCombatResource>,
    overlay_query: Query<Entity, With<PostCombatOverlay>>,
    fight_root: Query<Entity, With<FightScreenRoot>>,
    phase_state: Res<State<CombatPhaseState>>,
) {
    if !overlay_query.is_empty() {
        return;
    }

    let Some(combat) = combat_res.get() else {
        return;
    };
    let Ok(root_entity) = fight_root.get_single() else {
        return;
    };

    let is_victory = *phase_state.get() == CombatPhaseState::Victory;

    commands.entity(root_entity).with_children(|parent| {
        parent
            .spawn((
                PostCombatOverlay,
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(20.0),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            ))
            .with_children(|overlay| {
                let (message, color) = if is_victory {
                    ("VICTORY!", Color::srgb(0.9, 0.9, 0.3))
                } else {
                    ("DEFEAT...", Color::srgb(0.8, 0.3, 0.3))
                };

                overlay.spawn((
                    CombatResultText,
                    Text::new(message),
                    TextFont { font_size: 64.0, ..default() },
                    TextColor(color),
                ));

                if is_victory {
                    overlay.spawn((
                        RewardsText,
                        Text::new(format!("Gold: {} | XP: {}", combat.gold_gained, combat.xp_gained)),
                        TextFont { font_size: 24.0, ..default() },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));
                }

                overlay.spawn((
                    Text::new("What would you like to do?"),
                    TextFont { font_size: 20.0, ..default() },
                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    Node { margin: UiRect::top(Val::Px(20.0)), ..default() },
                ));

                spawn_post_combat_item(overlay, 0, "Fight Again");
                spawn_post_combat_item(overlay, 1, "Continue");
            });
    });
}

fn spawn_post_combat_item(parent: &mut ChildBuilder, index: usize, label: &str) {
    let selected = index == 0;
    parent.spawn((
        PostCombatMenuItem,
        MenuIndex(index),
        Text::new(label),
        TextFont { font_size: 28.0, ..default() },
        TextColor(nav_selection_text(selected)),
    ));
}

pub fn despawn_post_combat_overlay(
    mut commands: Commands,
    overlay_query: Query<Entity, With<PostCombatOverlay>>,
) {
    for entity in &overlay_query {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn update_combat_visuals(
    stats: Res<StatSheet>,
    combat_res: Res<ActiveCombatResource>,
    game_sprites: Res<GameSprites>,
    player_health_bar: Query<Entity, (With<PlayerHealthBar>, Without<EnemyHealthBar>)>,
    enemy_health_bar: Query<Entity, (With<EnemyHealthBar>, Without<PlayerHealthBar>)>,
    children: Query<&Children>,
    mut sprite_query: Query<&mut ImageNode, With<SpriteHealthBar>>,
    mut text_query: Query<&mut Text, With<HealthBarText>>,
) {
    let Some(sheet) = game_sprites.get(SpriteSheetKey::UiAll) else {
        return;
    };

    if let Ok(bar_entity) = player_health_bar.get_single() {
        update_health_bar(
            bar_entity,
            stats.hp(),
            stats.max_hp(),
            &children,
            &mut sprite_query,
            &mut text_query,
            sheet,
        );
    }

    if let Some(combat) = combat_res.get() {
        if let Ok(bar_entity) = enemy_health_bar.get_single() {
            let enemy_info = combat.enemy_info();
            update_health_bar(
                bar_entity,
                enemy_info.health,
                enemy_info.max_health,
                &children,
                &mut sprite_query,
                &mut text_query,
                sheet,
            );
        }
    }
}

pub fn cleanup_fight_screen(
    mut commands: Commands,
    fight_root: Query<Entity, With<FightScreenRoot>>,
) {
    if let Ok(entity) = fight_root.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

/// System to populate the fight background when the asset is ready.
pub fn populate_fight_background(
    mut commands: Commands,
    query: Query<Entity, With<NeedsFightBackground>>,
    selected_bg: Res<SelectedFightBackground>,
) {
    let Some(bg) = &selected_bg.0 else {
        return;
    };

    for entity in &query {
        commands
            .entity(entity)
            .remove::<NeedsFightBackground>()
            .remove::<BackgroundColor>()
            .insert(ImageNode::new(bg.clone()));
    }
}

/// System to populate the mob sprite when the asset is ready.
pub fn populate_mob_sprite(
    mut commands: Commands,
    query: Query<Entity, With<NeedsMobSprite>>,
    game_assets: Res<GameAssets>,
    combat_res: Res<ActiveCombatResource>,
) {
    let Some(combat) = combat_res.get() else {
        return;
    };

    let Some(sprite) = game_assets.sprites.mob_sprite(combat.mob.mob_id) else {
        return;
    };

    for entity in &query {
        commands
            .entity(entity)
            .remove::<NeedsMobSprite>()
            .insert(ImageNode::new(sprite.clone()));
    }
}

pub fn reset_fight_state(
    mut fight_state: ResMut<FightScreenState>,
    mut action_items: Query<(&MenuIndex, &mut TextColor, &mut Text), With<ActionMenuItem>>,
) {
    fight_state.reset();
    let labels = ["Attack", "Run"];
    for (menu_index, mut color, mut text) in action_items.iter_mut() {
        let selected = menu_index.0 == fight_state.action_selection;
        let suffix = if selected { " <" } else { "" };
        let text_color = if selected {
            Color::srgb(0.15, 0.1, 0.05)
        } else {
            Color::srgb(0.4, 0.35, 0.3)
        };
        *color = TextColor(text_color);
        **text = format!("{}{}", labels[menu_index.0], suffix);
    }
}

/// Updates the enemy name label when combat is initialized.
pub fn update_enemy_name(
    combat_res: Res<ActiveCombatResource>,
    mut name_query: Query<&mut Text, With<EnemyNameLabel>>,
) {
    let Some(combat) = combat_res.get() else {
        return;
    };

    let enemy_name = combat.enemy_info().name;
    for mut text in name_query.iter_mut() {
        **text = enemy_name.clone();
    }
}
