mod state;

use bevy::prelude::*;

use crate::combat::{enemy_attack_step, player_attack_step, process_defeat, process_victory, CombatPhase};
use crate::game::{ActiveCombatResource, PlayerResource};
use crate::input::{GameAction, NavigationDirection};
use crate::screens::shared::{spawn_combat_log, update_health_bar, CombatLogEntry};
use crate::states::AppState;
use crate::stats::HasStats;

pub use state::{CombatOrigin, CombatSource, FightScreenState};

/// Plugin that manages the fight screen.
pub struct FightPlugin;

impl Plugin for FightPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FightScreenState>()
            .init_resource::<CombatSource>()
            .init_resource::<CombatLogState>()
            .add_systems(OnEnter(AppState::Fight), (spawn_fight_screen, reset_fight_state).chain())
            .add_systems(OnExit(AppState::Fight), cleanup_fight_screen)
            .add_systems(
                Update,
                (
                    handle_fight_input,
                    execute_combat_turn,
                    update_combat_visuals,
                    handle_combat_end,
                )
                    .run_if(in_state(AppState::Fight)),
            );
    }
}

/// Resource tracking combat log entries for the current fight.
#[derive(Resource, Default)]
struct CombatLogState {
    entries: Vec<CombatLogEntry>,
}

/// Component marker for the fight screen UI root.
#[derive(Component)]
struct FightScreenRoot;

/// Component marker for action menu items.
#[derive(Component)]
struct ActionMenuItem {
    index: usize,
}

/// Component marker for post-combat menu items.
#[derive(Component)]
struct PostCombatMenuItem {
    index: usize,
}

/// Component marker for the player health bar.
#[derive(Component)]
struct PlayerHealthBar;

/// Component marker for the enemy health bar.
#[derive(Component)]
struct EnemyHealthBar;

/// Component marker for the combat log widget.
#[derive(Component)]
struct CombatLogContainer;

/// Component marker for the post-combat overlay.
#[derive(Component)]
struct PostCombatOverlay;

/// Component marker for victory/defeat message text.
#[derive(Component)]
struct CombatResultText;

/// Component marker for rewards display text.
#[derive(Component)]
struct RewardsText;

/// System to spawn the fight screen UI.
fn spawn_fight_screen(
    mut commands: Commands,
    player: Res<PlayerResource>,
    combat_res: Res<ActiveCombatResource>,
    mut log_state: ResMut<CombatLogState>,
) {
    // Initialize combat log with starting message
    log_state.entries.clear();
    if let Some(combat) = combat_res.get() {
        let enemy_info = combat.enemy_info();
        log_state.entries.push(CombatLogEntry::info(format!(
            "A wild {} appears!",
            enemy_info.name
        )));
    }

    // Get combat data for display
    let (player_health, player_max_health, enemy_name, enemy_health, enemy_max_health) =
        if let Some(combat) = combat_res.get() {
            let enemy_info = combat.enemy_info();
            (
                player.hp(),
                player.max_hp(),
                enemy_info.name.clone(),
                enemy_info.health,
                enemy_info.health, // Use current health as max for display
            )
        } else {
            // No combat - shouldn't happen, but provide defaults
            (player.hp(), player.max_hp(), "Unknown".to_string(), 1, 1)
        };

    // Root container
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
            // Top: Combatants section (Player vs Enemy)
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceAround,
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                })
                .with_children(|combatants| {
                    // Player side
                    combatants
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        })
                        .with_children(|player_side| {
                            player_side.spawn((
                                Text::new("PLAYER"),
                                TextFont {
                                    font_size: 24.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.5, 0.8, 0.5)),
                                Node {
                                    margin: UiRect::bottom(Val::Px(10.0)),
                                    ..default()
                                },
                            ));
                            // Spawn health bar with marker included
                            let player_name = player.name.to_string();
                            player_side.spawn((
                                PlayerHealthBar,
                                crate::screens::shared::HealthBar {
                                    entity_name: player_name.clone(),
                                },
                                Node {
                                    flex_direction: FlexDirection::Column,
                                    row_gap: Val::Px(5.0),
                                    width: Val::Px(200.0),
                                    ..default()
                                },
                            )).with_children(|bar| {
                                // Entity name
                                bar.spawn((
                                    Text::new(player_name.clone()),
                                    TextFont { font_size: 18.0, ..default() },
                                    TextColor(Color::WHITE),
                                ));
                                // Health bar background
                                bar.spawn((
                                    Node {
                                        width: Val::Percent(100.0),
                                        height: Val::Px(20.0),
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                                )).with_children(|bg| {
                                    let fill_percent = if player_max_health > 0 {
                                        (player_health as f32 / player_max_health as f32 * 100.0).clamp(0.0, 100.0)
                                    } else { 0.0 };
                                    bg.spawn((
                                        crate::screens::shared::HealthBarFill,
                                        Node {
                                            width: Val::Percent(fill_percent),
                                            height: Val::Percent(100.0),
                                            ..default()
                                        },
                                        BackgroundColor(Color::srgb(0.3, 0.8, 0.3)),
                                    ));
                                });
                                // HP text
                                bar.spawn((
                                    crate::screens::shared::HealthBarText,
                                    Text::new(format!("{}/{}", player_health, player_max_health)),
                                    TextFont { font_size: 14.0, ..default() },
                                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                                ));
                            });
                        });

                    // VS text
                    combatants.spawn((
                        Text::new("VS"),
                        TextFont {
                            font_size: 48.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));

                    // Enemy side
                    combatants
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        })
                        .with_children(|enemy_side| {
                            enemy_side.spawn((
                                Text::new("ENEMY"),
                                TextFont {
                                    font_size: 24.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.8, 0.5, 0.5)),
                                Node {
                                    margin: UiRect::bottom(Val::Px(10.0)),
                                    ..default()
                                },
                            ));
                            // Spawn enemy health bar with marker included
                            let enemy_name_clone = enemy_name.clone();
                            enemy_side.spawn((
                                EnemyHealthBar,
                                crate::screens::shared::HealthBar {
                                    entity_name: enemy_name_clone.clone(),
                                },
                                Node {
                                    flex_direction: FlexDirection::Column,
                                    row_gap: Val::Px(5.0),
                                    width: Val::Px(200.0),
                                    ..default()
                                },
                            )).with_children(|bar| {
                                bar.spawn((
                                    Text::new(enemy_name_clone.clone()),
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
                                    let fill_percent = if enemy_max_health > 0 {
                                        (enemy_health as f32 / enemy_max_health as f32 * 100.0).clamp(0.0, 100.0)
                                    } else { 0.0 };
                                    bg.spawn((
                                        crate::screens::shared::HealthBarFill,
                                        Node {
                                            width: Val::Percent(fill_percent),
                                            height: Val::Percent(100.0),
                                            ..default()
                                        },
                                        BackgroundColor(Color::srgb(0.8, 0.3, 0.3)),
                                    ));
                                });
                                bar.spawn((
                                    crate::screens::shared::HealthBarText,
                                    Text::new(format!("{}/{}", enemy_health, enemy_max_health)),
                                    TextFont { font_size: 14.0, ..default() },
                                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                                ));
                            });
                        });
                });

            // Middle: Combat log
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

            // Bottom: Action menu
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
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    ));

                    spawn_action_item(action_section, 0, "Attack");
                    spawn_action_item(action_section, 1, "Run");
                });
        });
}

/// Helper to spawn an action menu item.
fn spawn_action_item(parent: &mut ChildBuilder, index: usize, label: &str) {
    parent.spawn((
        ActionMenuItem { index },
        Text::new(label),
        TextFont {
            font_size: 28.0,
            ..default()
        },
        TextColor(Color::srgb(0.7, 0.7, 0.7)),
    ));
}

/// Helper to spawn a post-combat menu item.
fn spawn_post_combat_item(parent: &mut ChildBuilder, index: usize, label: &str) {
    parent.spawn((
        PostCombatMenuItem { index },
        Text::new(label),
        TextFont {
            font_size: 28.0,
            ..default()
        },
        TextColor(Color::srgb(0.7, 0.7, 0.7)),
    ));
}

/// System to handle input during combat.
fn handle_fight_input(
    mut action_reader: EventReader<GameAction>,
    mut fight_state: ResMut<FightScreenState>,
    combat_res: Res<ActiveCombatResource>,
    mut action_items: Query<(&ActionMenuItem, &mut TextColor)>,
    mut post_combat_items: Query<(&PostCombatMenuItem, &mut TextColor), Without<ActionMenuItem>>,
) {
    let is_combat_over = combat_res
        .get()
        .map(|c| c.is_combat_over())
        .unwrap_or(false);

    for action in action_reader.read() {
        if is_combat_over {
            // Post-combat navigation
            match action {
                GameAction::Navigate(NavigationDirection::Up) => {
                    fight_state.post_combat_up();
                    update_post_combat_visuals(&fight_state, &mut post_combat_items);
                }
                GameAction::Navigate(NavigationDirection::Down) => {
                    fight_state.post_combat_down();
                    update_post_combat_visuals(&fight_state, &mut post_combat_items);
                }
                _ => {}
            }
        } else {
            // In-combat navigation
            match action {
                GameAction::Navigate(NavigationDirection::Up) => {
                    fight_state.action_up();
                    update_action_visuals(&fight_state, &mut action_items);
                }
                GameAction::Navigate(NavigationDirection::Down) => {
                    fight_state.action_down();
                    update_action_visuals(&fight_state, &mut action_items);
                }
                _ => {}
            }
        }
    }
}

/// System to execute combat turns when player selects an action.
fn execute_combat_turn(
    mut action_reader: EventReader<GameAction>,
    fight_state: Res<FightScreenState>,
    mut combat_res: ResMut<ActiveCombatResource>,
    mut player: ResMut<PlayerResource>,
    mut log_state: ResMut<CombatLogState>,
    mut next_state: ResMut<NextState<AppState>>,
    combat_source: Res<CombatSource>,
) {
    // Only process if combat is active and not over
    let combat_active = combat_res.get().map(|c| !c.is_combat_over()).unwrap_or(false);
    if !combat_active {
        return;
    }

    for action in action_reader.read() {
        if *action == GameAction::Select {
            match fight_state.action_selection {
                0 => {
                    // Attack
                    if let Some(combat) = combat_res.get_mut() {
                        // Player attacks
                        let player_result = player_attack_step(&player, combat);
                        log_state.entries.push(CombatLogEntry::player_attack(
                            player_result.damage_to_target,
                            &player_result.defender,
                        ));

                        if player_result.target_died {
                            log_state.entries.push(CombatLogEntry::enemy_defeated(&player_result.defender));
                            // Process victory rewards
                            process_victory(&mut player, combat);
                        } else {
                            // Enemy attacks back
                            let enemy_result = enemy_attack_step(combat, &mut player);
                            log_state.entries.push(CombatLogEntry::enemy_attack(
                                enemy_result.damage_to_target,
                                &enemy_result.attacker,
                            ));

                            if enemy_result.target_died {
                                log_state.entries.push(CombatLogEntry::player_defeated());
                                // Process defeat
                                process_defeat(&mut player);
                            }
                        }
                    }
                }
                1 => {
                    // Run - return to origin
                    match combat_source.origin {
                        Some(CombatOrigin::Field) | None => {
                            next_state.set(AppState::Town);
                        }
                        Some(CombatOrigin::DungeonRoom) | Some(CombatOrigin::DungeonBoss) => {
                            next_state.set(AppState::Dungeon);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

/// System to update combat visuals (health bars).
fn update_combat_visuals(
    mut commands: Commands,
    player: Res<PlayerResource>,
    combat_res: Res<ActiveCombatResource>,
    player_health_bar: Query<Entity, With<PlayerHealthBar>>,
    enemy_health_bar: Query<Entity, With<EnemyHealthBar>>,
    children: Query<&Children>,
    mut fill_query: Query<&mut Node, With<crate::screens::shared::HealthBarFill>>,
    mut text_query: Query<&mut Text, With<crate::screens::shared::health_bar::HealthBarText>>,
) {
    // Update player health bar
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

    // Update enemy health bar
    if let Some(combat) = combat_res.get() {
        if let Ok(bar_entity) = enemy_health_bar.get_single() {
            let enemy_info = combat.enemy_info();
            update_health_bar(
                &mut commands,
                bar_entity,
                enemy_info.health,
                enemy_info.health, // Using current as max
                &children,
                &mut fill_query,
                &mut text_query,
            );
        }
    }
}

/// System to handle combat end state - show victory/defeat overlay.
fn handle_combat_end(
    mut commands: Commands,
    mut action_reader: EventReader<GameAction>,
    fight_state: Res<FightScreenState>,
    combat_res: Res<ActiveCombatResource>,
    mut next_state: ResMut<NextState<AppState>>,
    combat_source: Res<CombatSource>,
    overlay_query: Query<Entity, With<PostCombatOverlay>>,
    fight_root: Query<Entity, With<FightScreenRoot>>,
) {
    let combat_over = combat_res
        .get()
        .map(|c| c.is_combat_over())
        .unwrap_or(false);

    if !combat_over {
        return;
    }

    // Spawn overlay if it doesn't exist
    if overlay_query.is_empty() {
        if let Some(combat) = combat_res.get() {
            if let Ok(root_entity) = fight_root.get_single() {
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
                            // Result message
                            let (message, color) = if is_victory {
                                ("VICTORY!", Color::srgb(0.9, 0.9, 0.3))
                            } else {
                                ("DEFEAT...", Color::srgb(0.8, 0.3, 0.3))
                            };

                            overlay.spawn((
                                CombatResultText,
                                Text::new(message),
                                TextFont {
                                    font_size: 64.0,
                                    ..default()
                                },
                                TextColor(color),
                            ));

                            // Rewards (if victory)
                            if is_victory {
                                overlay.spawn((
                                    RewardsText,
                                    Text::new(format!(
                                        "Gold: {} | XP: {}",
                                        combat.gold_gained, combat.xp_gained
                                    )),
                                    TextFont {
                                        font_size: 24.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                                ));
                            }

                            // Menu options
                            overlay.spawn((
                                Text::new("What would you like to do?"),
                                TextFont {
                                    font_size: 20.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                                Node {
                                    margin: UiRect::top(Val::Px(20.0)),
                                    ..default()
                                },
                            ));

                            spawn_post_combat_item(overlay, 0, "Fight Again");
                            spawn_post_combat_item(overlay, 1, "Continue");
                        });
                });
            }
        }
    }

    // Handle post-combat menu selection
    for action in action_reader.read() {
        if *action == GameAction::Select {
            match fight_state.post_combat_selection {
                0 => {
                    // Fight Again - stay in fight state, will need new combat
                    // For now, return to origin (proper implementation would respawn combat)
                    match combat_source.origin {
                        Some(CombatOrigin::Field) | None => {
                            next_state.set(AppState::Town);
                        }
                        Some(CombatOrigin::DungeonRoom) | Some(CombatOrigin::DungeonBoss) => {
                            next_state.set(AppState::Dungeon);
                        }
                    }
                }
                1 => {
                    // Continue - return to origin
                    match combat_source.origin {
                        Some(CombatOrigin::Field) | None => {
                            next_state.set(AppState::Town);
                        }
                        Some(CombatOrigin::DungeonRoom) | Some(CombatOrigin::DungeonBoss) => {
                            next_state.set(AppState::Dungeon);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

/// Update action menu visuals based on current selection.
fn update_action_visuals(
    state: &FightScreenState,
    items: &mut Query<(&ActionMenuItem, &mut TextColor)>,
) {
    for (item, mut color) in items.iter_mut() {
        if item.index == state.action_selection {
            *color = TextColor(Color::srgb(1.0, 1.0, 1.0));
        } else {
            *color = TextColor(Color::srgb(0.7, 0.7, 0.7));
        }
    }
}

/// Update post-combat menu visuals based on current selection.
fn update_post_combat_visuals(
    state: &FightScreenState,
    items: &mut Query<(&PostCombatMenuItem, &mut TextColor), Without<ActionMenuItem>>,
) {
    for (item, mut color) in items.iter_mut() {
        if item.index == state.post_combat_selection {
            *color = TextColor(Color::srgb(1.0, 1.0, 1.0));
        } else {
            *color = TextColor(Color::srgb(0.7, 0.7, 0.7));
        }
    }
}

/// System to reset fight state when entering the screen.
fn reset_fight_state(
    mut fight_state: ResMut<FightScreenState>,
    mut action_items: Query<(&ActionMenuItem, &mut TextColor)>,
) {
    fight_state.reset();
    update_action_visuals(&fight_state, &mut action_items);
}

/// System to cleanup fight screen UI.
fn cleanup_fight_screen(
    mut commands: Commands,
    fight_root: Query<Entity, With<FightScreenRoot>>,
) {
    if let Ok(entity) = fight_root.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}
