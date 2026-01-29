use bevy::prelude::*;

use super::super::components::{SmoothPosition, TileSizes};
use super::super::constants::MOVE_SPEED;

pub fn interpolate_positions(
    time: Res<Time>,
    tile_sizes: Option<Res<TileSizes>>,
    mut query: Query<(&mut SmoothPosition, &mut Node)>,
) {
    let Some(tile_sizes) = tile_sizes else {
        return;
    };
    let tile_size = tile_sizes.tile_size;
    let speed = MOVE_SPEED * tile_size;

    for (mut pos, mut node) in &mut query {
        if !pos.moving {
            continue;
        }

        let delta = pos.target - pos.current;
        let distance = delta.length();

        if distance < 0.5 {
            pos.current = pos.target;
            pos.moving = false;
        } else {
            let step = speed * time.delta_secs();
            pos.current += delta.normalize() * step.min(distance);
        }

        node.left = Val::Px(pos.current.x);
        node.top = Val::Px(pos.current.y);
    }
}
