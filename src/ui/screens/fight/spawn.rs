use bevy::prelude::*;

use crate::combat::ActiveCombatResource;
use crate::game::PlayerName;
use crate::stats::{HasStats, StatSheet};
use crate::ui::{HealthBarBundle, HealthBarNameBundle, HealthBarTextBundle, SpriteHealthBarBundle};

use super::components::{
    ActionMenuItem, EnemyHealthBar, EnemyNameLabel, FightScreenRoot, NeedsFightBackground,
    NeedsFightPopup, NeedsMobSprite, PlayerHealthBar,
};
use super::styles::{action_label, action_text_color};
use super::systems::SelectedFightBackground;
use crate::ui::MenuIndex;

pub fn spawn_fight_screen(
    mut commands: Commands,
    name: Res<PlayerName>,
    stats: Res<StatSheet>,
    combat_res: Res<ActiveCombatResource>,
    mut selected_bg: ResMut<SelectedFightBackground>,
) {
    // Select a random background (1-80)
    let bg_index = rand::random::<u32>() % 80 + 1;
    selected_bg.0 = Some(format!("Background_{}", bg_index));

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

            // Action menu popup in bottom right (populated by system when sprite loads)
            parent
                .spawn((
                    NeedsFightPopup,
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
                                width: Val::Px(224.0),
                                height: Val::Px(224.0),
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
                                    width: Val::Px(192.0),
                                    height: Val::Px(192.0),
                                    ..default()
                                },
                            ));
                        });
                });
        });
}

fn spawn_action_item(parent: &mut ChildBuilder, index: usize, label: &str, selected: bool) {
    parent.spawn((
        ActionMenuItem,
        MenuIndex(index),
        Text::new(action_label(label, selected)),
        TextFont { font_size: 18.0, ..default() },
        TextColor(action_text_color(selected)),
    ));
}
