use bevy::prelude::*;

use crate::combat::{ActiveCombatResource, CombatLogState, CombatPhase};
use crate::game::Player;
use crate::screens::shared::{spawn_combat_log, update_health_bar, HealthBar, HealthBarFill, HealthBarText};
use crate::stats::HasStats;
use crate::ui::{nav_selection_text, MenuIndex};

use super::components::*;
use super::state::FightScreenState;

pub fn spawn_fight_screen(
    mut commands: Commands,
    player: Res<Player>,
    combat_res: Res<ActiveCombatResource>,
    log_state: Res<CombatLogState>,
) {
    let (player_health, player_max_health, enemy_name, enemy_health, enemy_max_health) =
        if let Some(combat) = combat_res.get() {
            let enemy_info = combat.enemy_info();
            (
                player.hp(),
                player.max_hp(),
                enemy_info.name.clone(),
                enemy_info.health,
                enemy_info.max_health,
            )
        } else {
            (player.hp(), player.max_hp(), "Unknown".to_string(), 1, 1)
        };

    commands
        .spawn((
            FightScreenRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
        ))
        .with_children(|parent| {
            spawn_combatants_section(parent, &player, player_health, player_max_health, &enemy_name, enemy_health, enemy_max_health);
            spawn_combat_log_section(parent, &log_state);
            spawn_action_menu(parent);
        });
}

fn spawn_combatants_section(
    parent: &mut ChildBuilder,
    player: &Player,
    player_health: i32,
    player_max_health: i32,
    enemy_name: &str,
    enemy_health: i32,
    enemy_max_health: i32,
) {
    parent
        .spawn(Node {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceAround,
            margin: UiRect::bottom(Val::Px(20.0)),
            ..default()
        })
        .with_children(|combatants| {
            spawn_player_side(combatants, player, player_health, player_max_health);

            combatants.spawn((
                Text::new("VS"),
                TextFont { font_size: 48.0, ..default() },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));

            spawn_enemy_side(combatants, enemy_name, enemy_health, enemy_max_health);
        });
}

fn spawn_player_side(parent: &mut ChildBuilder, player: &Player, health: i32, max_health: i32) {
    let player_name = player.name.to_string();
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|player_side| {
            player_side.spawn((
                Text::new("PLAYER"),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::srgb(0.5, 0.8, 0.5)),
                Node { margin: UiRect::bottom(Val::Px(10.0)), ..default() },
            ));

            player_side.spawn((
                PlayerHealthBar,
                HealthBar,
                Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(5.0),
                    width: Val::Px(200.0),
                    ..default()
                },
            )).with_children(|bar| {
                bar.spawn((
                    Text::new(player_name),
                    TextFont { font_size: 18.0, ..default() },
                    TextColor(Color::WHITE),
                ));
                bar.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(20.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                )).with_children(|bg| {
                    let fill_percent = if max_health > 0 {
                        (health as f32 / max_health as f32 * 100.0).clamp(0.0, 100.0)
                    } else { 0.0 };
                    bg.spawn((
                        HealthBarFill,
                        Node {
                            width: Val::Percent(fill_percent),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.3, 0.8, 0.3)),
                    ));
                });
                bar.spawn((
                    HealthBarText,
                    Text::new(format!("{}/{}", health, max_health)),
                    TextFont { font_size: 14.0, ..default() },
                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                ));
            });
        });
}

fn spawn_enemy_side(parent: &mut ChildBuilder, enemy_name: &str, health: i32, max_health: i32) {
    let name = enemy_name.to_string();
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|enemy_side| {
            enemy_side.spawn((
                Text::new("ENEMY"),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::srgb(0.8, 0.5, 0.5)),
                Node { margin: UiRect::bottom(Val::Px(10.0)), ..default() },
            ));

            enemy_side.spawn((
                EnemyHealthBar,
                HealthBar,
                Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(5.0),
                    width: Val::Px(200.0),
                    ..default()
                },
            )).with_children(|bar| {
                bar.spawn((
                    Text::new(name),
                    TextFont { font_size: 18.0, ..default() },
                    TextColor(Color::WHITE),
                ));
                bar.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(20.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                )).with_children(|bg| {
                    let fill_percent = if max_health > 0 {
                        (health as f32 / max_health as f32 * 100.0).clamp(0.0, 100.0)
                    } else { 0.0 };
                    bg.spawn((
                        HealthBarFill,
                        Node {
                            width: Val::Percent(fill_percent),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.8, 0.3, 0.3)),
                    ));
                });
                bar.spawn((
                    HealthBarText,
                    Text::new(format!("{}/{}", health, max_health)),
                    TextFont { font_size: 14.0, ..default() },
                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                ));
            });
        });
}

fn spawn_combat_log_section(parent: &mut ChildBuilder, log_state: &CombatLogState) {
    parent
        .spawn((
            CombatLogContainer,
            Node {
                width: Val::Percent(100.0),
                margin: UiRect::vertical(Val::Px(20.0)),
                ..default()
            },
        ))
        .with_children(|log_parent| {
            spawn_combat_log(log_parent, &log_state.entries, 6);
        });
}

fn spawn_action_menu(parent: &mut ChildBuilder) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(10.0),
            ..default()
        })
        .with_children(|action_section| {
            action_section.spawn((
                Text::new("Choose Action:"),
                TextFont { font_size: 20.0, ..default() },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ));

            spawn_action_item(action_section, 0, "Attack", true);
            spawn_action_item(action_section, 1, "Run", false);
        });
}

fn spawn_action_item(parent: &mut ChildBuilder, index: usize, label: &str, selected: bool) {
    let prefix = if selected { ">" } else { " " };
    parent.spawn((
        ActionMenuItem,
        MenuIndex(index),
        Text::new(format!("{} {}", prefix, label)),
        TextFont { font_size: 28.0, ..default() },
        TextColor(nav_selection_text(selected)),
    ));
}

pub fn spawn_post_combat_overlay(
    mut commands: Commands,
    combat_res: Res<ActiveCombatResource>,
    overlay_query: Query<Entity, With<PostCombatOverlay>>,
    fight_root: Query<Entity, With<FightScreenRoot>>,
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

    let is_victory = combat.phase == CombatPhase::Victory;

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
    mut commands: Commands,
    player: Res<Player>,
    combat_res: Res<ActiveCombatResource>,
    player_health_bar: Query<Entity, (With<PlayerHealthBar>, Without<EnemyHealthBar>)>,
    enemy_health_bar: Query<Entity, (With<EnemyHealthBar>, Without<PlayerHealthBar>)>,
    children: Query<&Children>,
    mut fill_query: Query<&mut Node, With<HealthBarFill>>,
    mut text_query: Query<&mut Text, With<HealthBarText>>,
) {
    if let Ok(bar_entity) = player_health_bar.get_single() {
        update_health_bar(
            &mut commands,
            bar_entity,
            player.hp(),
            player.max_hp(),
            &children,
            &mut fill_query,
            &mut text_query,
        );
    }

    if let Some(combat) = combat_res.get() {
        if let Ok(bar_entity) = enemy_health_bar.get_single() {
            let enemy_info = combat.enemy_info();
            update_health_bar(
                &mut commands,
                bar_entity,
                enemy_info.health,
                enemy_info.max_health,
                &children,
                &mut fill_query,
                &mut text_query,
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

pub fn reset_fight_state(
    mut fight_state: ResMut<FightScreenState>,
    mut action_items: Query<(&MenuIndex, &mut TextColor, &mut Text), With<ActionMenuItem>>,
) {
    fight_state.reset();
    let labels = ["Attack", "Run"];
    for (menu_index, mut color, mut text) in action_items.iter_mut() {
        let selected = menu_index.0 == fight_state.action_selection;
        let prefix = if selected { ">" } else { " " };
        *color = TextColor(nav_selection_text(selected));
        **text = format!("{} {}", prefix, labels[menu_index.0]);
    }
}
