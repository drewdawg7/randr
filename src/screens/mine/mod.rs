mod grid;
mod state;

use bevy::prelude::*;

use crate::game::Player;
use crate::input::{GameAction, NavigationDirection};
use crate::inventory::ManagesItems;
use crate::location::mine::RockId;
use crate::stats::StatType;
use crate::states::AppState;

pub use state::MineScreenState;
use state::MineTile;

use grid::{spawn_grid, update_grid_tiles, update_player_sprite, GridTile, PlayerSprite};

/// Plugin that manages the mine screen.
pub struct MinePlugin;

impl Plugin for MinePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MineScreenState>()
            .add_systems(OnEnter(AppState::Mine), (spawn_mine_screen, reset_mine_state).chain())
            .add_systems(OnExit(AppState::Mine), cleanup_mine_screen)
            .add_systems(
                Update,
                (
                    handle_mine_input,
                    handle_mining_action,
                    handle_ladder_exit,
                    update_player_sprite,
                    update_grid_tiles,
                    update_message_display,
                )
                    .run_if(in_state(AppState::Mine)),
            );
    }
}

/// Component marker for the mine screen UI root.
#[derive(Component)]
struct MineScreenRoot;

/// Component marker for the message text display.
#[derive(Component)]
struct MessageText;

/// System to spawn the mine screen.
fn spawn_mine_screen(mut commands: Commands, state: Res<MineScreenState>) {
    // Spawn the grid
    spawn_grid(&mut commands, &state);

    // Spawn UI overlay for messages
    commands
        .spawn((
            MineScreenRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            // No background color - let the grid show through
            BackgroundColor(Color::NONE),
        ))
        .with_children(|parent| {
            // Top: Title
            parent.spawn((
                Text::new("MINE"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
            ));

            // Middle: Message display
            parent.spawn((
                MessageText,
                Text::new(""),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.5)),
            ));

            // Bottom: Instructions
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(5.0),
                    ..default()
                })
                .with_children(|instructions| {
                    instructions.spawn((
                        Text::new("Arrow Keys: Move | Space: Mine | Backspace: Exit at Ladder"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                    ));
                });
        });
}

/// System to handle input during mining.
fn handle_mine_input(
    mut action_reader: EventReader<GameAction>,
    mut state: ResMut<MineScreenState>,
) {
    for action in action_reader.read() {
        match action {
            GameAction::Navigate(direction) => {
                // Clear message when moving
                state.clear_message();

                let (dx, dy) = match direction {
                    NavigationDirection::Up => (0, -1),
                    NavigationDirection::Down => (0, 1),
                    NavigationDirection::Left => (-1, 0),
                    NavigationDirection::Right => (1, 0),
                };

                state.move_player(dx, dy);
            }
            _ => {}
        }
    }
}

/// System to handle mining actions.
fn handle_mining_action(
    mut action_reader: EventReader<GameAction>,
    mut state: ResMut<MineScreenState>,
    mut player: ResMut<Player>,
) {
    for action in action_reader.read() {
        if *action == GameAction::Mine {
            // Find adjacent mineable tiles
            let adjacent = state.get_adjacent_mineable();

            if adjacent.is_empty() {
                state.set_message("No rocks nearby to mine!".to_string());
                continue;
            }

            // Mine the first adjacent rock (could be extended to choose direction)
            let (mx, my) = adjacent[0];
            let tile = state.grid.get(mx, my);

            if let Some(tile) = tile {
                if tile.is_mineable() {
                    // Determine which rock to mine
                    let rock_id = match tile {
                        MineTile::Ore(ore_type) => ore_type.rock_id(),
                        MineTile::Rock => {
                            // Random rock type for generic rocks
                            use rand::Rng;
                            let mut rng = rand::thread_rng();
                            let roll = rng.gen_range(0..3);
                            match roll {
                                0 => RockId::Copper,
                                1 => RockId::Tin,
                                _ => RockId::Coal,
                            }
                        }
                        _ => continue,
                    };

                    // Get the rock and roll for loot
                    let rock = rock_id.spawn();
                    let magic_find = player.stats.value(StatType::MagicFind);
                    let drops = rock.loot.roll_drops(magic_find);

                    // Add items to player inventory
                    let mut message_parts = Vec::new();
                    for drop in drops {
                        let item_name = drop.item.name.clone();
                        let quantity = drop.quantity;

                        // Add item to inventory
                        for _ in 0..quantity {
                            if let Ok(_result) = player.add_to_inv(drop.item.clone()) {
                                message_parts.push(format!("{}x {}", quantity, item_name));
                                break; // Only add message once per drop type
                            }
                        }
                    }

                    // Set message
                    if message_parts.is_empty() {
                        state.set_message("Just rocks...".to_string());
                    } else {
                        state.set_message(format!("Found: {}", message_parts.join(", ")));
                    }

                    // Replace mined tile with floor
                    state.grid.set(mx, my, MineTile::Floor);
                }
            }
        }
    }
}

/// System to handle exiting via ladder.
fn handle_ladder_exit(
    mut action_reader: EventReader<GameAction>,
    state: Res<MineScreenState>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for action in action_reader.read() {
        if *action == GameAction::Back {
            if state.is_on_ladder() {
                next_state.set(AppState::Town);
            }
        }
    }
}

/// System to update the message display.
fn update_message_display(
    state: Res<MineScreenState>,
    mut message_query: Query<&mut Text, With<MessageText>>,
) {
    if let Ok(mut text) = message_query.get_single_mut() {
        if let Some(ref message) = state.mining_message {
            **text = message.clone();
        } else {
            **text = String::new();
        }
    }
}

/// System to reset mine state when entering the screen.
fn reset_mine_state(mut state: ResMut<MineScreenState>) {
    *state = MineScreenState::new();
}

/// System to cleanup mine screen.
fn cleanup_mine_screen(
    mut commands: Commands,
    ui_root: Query<Entity, With<MineScreenRoot>>,
    grid_tiles: Query<Entity, With<GridTile>>,
    player_sprite: Query<Entity, With<PlayerSprite>>,
) {
    // Cleanup UI
    if let Ok(entity) = ui_root.get_single() {
        commands.entity(entity).despawn_recursive();
    }

    // Cleanup grid tiles
    for entity in grid_tiles.iter() {
        commands.entity(entity).despawn();
    }

    // Cleanup player sprite
    if let Ok(entity) = player_sprite.get_single() {
        commands.entity(entity).despawn();
    }
}
