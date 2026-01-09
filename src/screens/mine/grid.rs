use bevy::prelude::*;

use super::state::MineScreenState;

/// Component marker for grid tiles.
#[derive(Component)]
pub struct GridTile {
    pub x: usize,
    pub y: usize,
}

/// Component marker for the player sprite in the mine.
#[derive(Component)]
pub struct PlayerSprite;

/// Size of each tile in pixels.
const TILE_SIZE: f32 = 40.0;

/// Spawn the mine grid with colored sprites.
pub fn spawn_grid(commands: &mut Commands, state: &MineScreenState) {
    let grid = &state.grid;

    // Calculate offset to center the grid
    let grid_width = grid.width as f32 * TILE_SIZE;
    let grid_height = grid.height as f32 * TILE_SIZE;
    let offset_x = -grid_width / 2.0 + TILE_SIZE / 2.0;
    let offset_y = grid_height / 2.0 - TILE_SIZE / 2.0;

    // Spawn tiles
    for y in 0..grid.height {
        for x in 0..grid.width {
            let tile = grid.tiles[y][x];
            let color = tile.color();

            let pos_x = offset_x + x as f32 * TILE_SIZE;
            let pos_y = offset_y - y as f32 * TILE_SIZE;

            commands.spawn((
                GridTile { x, y },
                Sprite {
                    color,
                    custom_size: Some(Vec2::new(TILE_SIZE - 2.0, TILE_SIZE - 2.0)),
                    ..default()
                },
                Transform::from_xyz(pos_x, pos_y, 0.0),
            ));
        }
    }

    // Spawn player sprite
    let (px, py) = state.player_pos;
    let pos_x = offset_x + px as f32 * TILE_SIZE;
    let pos_y = offset_y - py as f32 * TILE_SIZE;

    commands.spawn((
        PlayerSprite,
        Sprite {
            color: Color::srgb(0.2, 0.8, 0.2), // Green for player
            custom_size: Some(Vec2::new(TILE_SIZE - 8.0, TILE_SIZE - 8.0)),
            ..default()
        },
        Transform::from_xyz(pos_x, pos_y, 1.0), // Higher z to render on top
    ));
}

/// Update the player sprite position.
pub fn update_player_sprite(
    state: Res<MineScreenState>,
    mut player_query: Query<&mut Transform, With<PlayerSprite>>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let grid = &state.grid;
        let (px, py) = state.player_pos;

        // Calculate offset to center the grid (same as spawn_grid)
        let grid_width = grid.width as f32 * TILE_SIZE;
        let grid_height = grid.height as f32 * TILE_SIZE;
        let offset_x = -grid_width / 2.0 + TILE_SIZE / 2.0;
        let offset_y = grid_height / 2.0 - TILE_SIZE / 2.0;

        let pos_x = offset_x + px as f32 * TILE_SIZE;
        let pos_y = offset_y - py as f32 * TILE_SIZE;

        transform.translation = Vec3::new(pos_x, pos_y, 1.0);
    }
}

/// Update grid tile sprites when tiles change (e.g., after mining).
pub fn update_grid_tiles(
    state: Res<MineScreenState>,
    mut tile_query: Query<(&GridTile, &mut Sprite)>,
) {
    for (grid_tile, mut sprite) in tile_query.iter_mut() {
        if let Some(tile) = state.grid.get(grid_tile.x, grid_tile.y) {
            sprite.color = tile.color();
        }
    }
}
