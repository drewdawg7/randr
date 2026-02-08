use avian2d::prelude::*;
use bevy::prelude::*;

use crate::dungeon::{MovementConfig, TileWorldSize};

use super::super::components::{DungeonPlayer, FacingDirection};

pub fn update_player_sprite_direction(
    mut query: Query<(&LinearVelocity, &mut Sprite, &mut FacingDirection), With<DungeonPlayer>>,
    movement: Res<MovementConfig>,
    tile_size: Res<TileWorldSize>,
) {
    let threshold = movement.flip_threshold(tile_size.0);

    for (velocity, mut sprite, mut facing) in &mut query {
        if velocity.x < -threshold {
            sprite.flip_x = true;
            *facing = FacingDirection::Left;
        } else if velocity.x > threshold {
            sprite.flip_x = false;
            *facing = FacingDirection::Right;
        }
    }
}
