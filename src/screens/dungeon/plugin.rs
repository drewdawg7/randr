use bevy::prelude::*;

use crate::assets::{GameSprites, SpriteSheetKey};
use crate::dungeon::{DungeonEntity, DungeonLayout, LayoutId, TileRenderer, TileType};
use crate::input::{GameAction, NavigationDirection};
use crate::states::AppState;
use crate::ui::widgets::PlayerStats;
use crate::ui::{DungeonMobSprite, DungeonPlayerSprite};

/// Resource tracking dungeon state for player movement.
#[derive(Resource)]
pub struct DungeonState {
    pub layout: DungeonLayout,
    pub player_pos: (usize, usize),
}

/// Marker component for grid cells with their coordinates.
#[derive(Component)]
pub struct DungeonCell {
    pub x: usize,
    pub y: usize,
}

/// Marker component for the player entity in the dungeon.
#[derive(Component)]
pub struct DungeonPlayer;

/// Marker for dungeon screen root
#[derive(Component)]
struct DungeonRoot;

pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Dungeon), spawn_dungeon_screen)
            .add_systems(OnExit(AppState::Dungeon), cleanup_dungeon)
            .add_systems(
                Update,
                (handle_dungeon_movement, handle_back_action).run_if(in_state(AppState::Dungeon)),
            );
    }
}

fn spawn_dungeon_screen(mut commands: Commands, game_sprites: Res<GameSprites>) {
    let Some(tile_sheet) = game_sprites.get(SpriteSheetKey::DungeonTileset) else {
        return;
    };

    const TILE_SIZE: f32 = 48.0;

    let layout = LayoutId::StartingRoom.layout();

    // Find player spawn position
    let player_pos = layout
        .iter()
        .find(|(_, _, tile)| tile.tile_type == TileType::PlayerSpawn)
        .map(|(x, y, _)| (x, y))
        .unwrap_or((0, 0));

    // Insert dungeon state resource
    commands.insert_resource(DungeonState {
        layout: layout.clone(),
        player_pos,
    });

    // Spawn root UI
    commands
        .spawn((
            DungeonRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
        ))
        .with_children(|parent| {
            // Player stats banner at top
            parent.spawn(PlayerStats);

            // Content area
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(20.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                })
                .with_children(|content| {
                    // Dungeon grid
                    content
                        .spawn(Node {
                            display: Display::Grid,
                            grid_template_columns: vec![GridTrack::px(TILE_SIZE); layout.width()],
                            grid_template_rows: vec![GridTrack::px(TILE_SIZE); layout.height()],
                            ..default()
                        })
                        .with_children(|grid| {
                            for y in 0..layout.height() {
                                for x in 0..layout.width() {
                                    grid.spawn((
                                        DungeonCell { x, y },
                                        Node {
                                            width: Val::Px(TILE_SIZE),
                                            height: Val::Px(TILE_SIZE),
                                            ..default()
                                        },
                                    ))
                                    .with_children(|cell| {
                                        // Spawn tile background
                                        if let Some((slice, flip_x)) =
                                            TileRenderer::resolve(&layout, x, y)
                                        {
                                            if let Some(mut img) =
                                                tile_sheet.image_node(slice.as_str())
                                            {
                                                if flip_x {
                                                    img.flip_x = true;
                                                }
                                                cell.spawn((
                                                    img,
                                                    Node {
                                                        position_type: PositionType::Absolute,
                                                        width: Val::Px(TILE_SIZE),
                                                        height: Val::Px(TILE_SIZE),
                                                        ..default()
                                                    },
                                                ));
                                            }
                                        }

                                        // Spawn player sprite at PlayerSpawn tile
                                        if let Some(tile) = layout.tile_at(x, y) {
                                            if tile.tile_type == TileType::PlayerSpawn {
                                                cell.spawn((
                                                    DungeonPlayer,
                                                    DungeonPlayerSprite,
                                                    Node {
                                                        position_type: PositionType::Absolute,
                                                        width: Val::Px(TILE_SIZE),
                                                        height: Val::Px(TILE_SIZE),
                                                        ..default()
                                                    },
                                                ));
                                            }
                                        }

                                        // Spawn entity if present
                                        if let Some(entity) = layout.entity_at(x, y) {
                                            let entity_node = Node {
                                                position_type: PositionType::Absolute,
                                                width: Val::Px(TILE_SIZE),
                                                height: Val::Px(TILE_SIZE),
                                                ..default()
                                            };

                                            match entity {
                                                DungeonEntity::Chest { .. } => {
                                                    if let Some(entity_sheet) =
                                                        game_sprites.get(entity.sprite_sheet_key())
                                                    {
                                                        if let Some(img) = entity_sheet
                                                            .image_node(entity.sprite_name())
                                                        {
                                                            cell.spawn((img, entity_node));
                                                        }
                                                    }
                                                }
                                                DungeonEntity::Mob { mob_id } => {
                                                    cell.spawn((
                                                        DungeonMobSprite { mob_id: *mob_id },
                                                        entity_node,
                                                    ));
                                                }
                                            }
                                        }
                                    });
                                }
                            }
                        });
                });
        });
}

/// Handle arrow key movement in the dungeon.
fn handle_dungeon_movement(
    mut commands: Commands,
    mut action_reader: EventReader<GameAction>,
    mut state: ResMut<DungeonState>,
    player_query: Query<Entity, With<DungeonPlayer>>,
    cell_query: Query<(Entity, &DungeonCell)>,
) {
    let Ok(player_entity) = player_query.get_single() else {
        return;
    };

    for action in action_reader.read() {
        let GameAction::Navigate(direction) = action else {
            continue;
        };

        let (dx, dy): (i32, i32) = match direction {
            NavigationDirection::Up => (0, -1),
            NavigationDirection::Down => (0, 1),
            NavigationDirection::Left => (-1, 0),
            NavigationDirection::Right => (1, 0),
        };

        let (cur_x, cur_y) = state.player_pos;
        let new_x = (cur_x as i32 + dx).max(0) as usize;
        let new_y = (cur_y as i32 + dy).max(0) as usize;

        // Check if target tile is a Floor
        let is_floor = state
            .layout
            .tile_at(new_x, new_y)
            .map(|t| t.tile_type == TileType::Floor)
            .unwrap_or(false);

        // Check if target tile has no entity
        let no_entity = state.layout.entity_at(new_x, new_y).is_none();

        if is_floor && no_entity {
            state.player_pos = (new_x, new_y);

            if let Some((cell_entity, _)) = cell_query
                .iter()
                .find(|(_, cell)| cell.x == new_x && cell.y == new_y)
            {
                commands.entity(player_entity).set_parent(cell_entity);
            }
        }
    }
}

fn handle_back_action(
    mut action_events: EventReader<GameAction>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for action in action_events.read() {
        if matches!(action, GameAction::Back) {
            next_state.set(AppState::Menu);
        }
    }
}

fn cleanup_dungeon(mut commands: Commands, query: Query<Entity, With<DungeonRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
    commands.remove_resource::<DungeonState>();
}
