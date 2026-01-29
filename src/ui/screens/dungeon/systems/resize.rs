use bevy::prelude::*;
use bevy::window::WindowResized;

use super::super::components::{
    DungeonContainer, DungeonGrid, DungeonPlayer, EntityLayer, SmoothPosition, UiScale,
};
use super::super::constants::{BASE_TILE, ENTITY_VISUAL_SCALE};
use crate::dungeon::{DungeonEntity, DungeonEntityMarker, DungeonState};
use crate::ui::MobSpriteSheets;

pub fn handle_window_resize(
    mut resize_events: EventReader<WindowResized>,
    windows: Query<&Window>,
    state: Res<DungeonState>,
    mob_sheets: Res<MobSpriteSheets>,
    scale: Option<ResMut<UiScale>>,
    mut grid_query: Query<&mut Node, With<DungeonGrid>>,
    mut container_query: Query<
        &mut Node,
        (
            With<DungeonContainer>,
            Without<DungeonGrid>,
            Without<DungeonPlayer>,
            Without<EntityLayer>,
        ),
    >,
    mut layer_query: Query<
        &mut Node,
        (
            With<EntityLayer>,
            Without<DungeonGrid>,
            Without<DungeonContainer>,
            Without<DungeonPlayer>,
        ),
    >,
    mut player_query: Query<
        (&mut Node, &mut SmoothPosition),
        (
            With<DungeonPlayer>,
            Without<DungeonGrid>,
            Without<DungeonContainer>,
            Without<EntityLayer>,
        ),
    >,
    mut entity_query: Query<
        (&DungeonEntityMarker, &mut Node),
        (
            Without<DungeonPlayer>,
            Without<DungeonGrid>,
            Without<DungeonContainer>,
            Without<EntityLayer>,
        ),
    >,
) {
    let Some(layout) = state.layout.as_ref() else {
        return;
    };

    let Some(mut scale) = scale else {
        return;
    };

    for event in resize_events.read() {
        let Ok(window) = windows.get(event.window) else {
            continue;
        };

        let new_scale = UiScale::calculate(window.height());

        if new_scale != scale.0 {
            scale.0 = new_scale;
            let tile_size = BASE_TILE * new_scale as f32;
            let grid_width = tile_size * layout.width() as f32;
            let grid_height = tile_size * layout.height() as f32;

            if let Ok(mut grid_node) = grid_query.get_single_mut() {
                grid_node.grid_template_columns = vec![GridTrack::px(tile_size); layout.width()];
                grid_node.grid_template_rows = vec![GridTrack::px(tile_size); layout.height()];
            }

            if let Ok(mut container_node) = container_query.get_single_mut() {
                container_node.width = Val::Px(grid_width);
                container_node.height = Val::Px(grid_height);
            }

            if let Ok(mut layer_node) = layer_query.get_single_mut() {
                layer_node.width = Val::Px(grid_width);
                layer_node.height = Val::Px(grid_height);
            }

            if let Ok((mut player_node, mut smooth_pos)) = player_query.get_single_mut() {
                let entity_sprite_size = ENTITY_VISUAL_SCALE * tile_size;
                let entity_offset = -(entity_sprite_size - tile_size) / 2.0;
                let new_px = Vec2::new(
                    state.player_pos.x as f32 * tile_size + entity_offset,
                    state.player_pos.y as f32 * tile_size + entity_offset,
                );
                smooth_pos.current = new_px;
                smooth_pos.target = new_px;
                smooth_pos.moving = false;
                player_node.left = Val::Px(new_px.x);
                player_node.top = Val::Px(new_px.y);
                player_node.width = Val::Px(entity_sprite_size);
                player_node.height = Val::Px(entity_sprite_size);
            }

            let entity_sprite_size = ENTITY_VISUAL_SCALE * tile_size;
            for (marker, mut entity_node) in entity_query.iter_mut() {
                let (visual_width, visual_height) = match marker.entity_type {
                    DungeonEntity::Mob { mob_id, .. } => {
                        let frame_size = mob_sheets
                            .get(mob_id)
                            .map(|s| s.frame_size)
                            .unwrap_or(UVec2::splat(32));
                        let aspect = frame_size.x as f32 / frame_size.y as f32;
                        (entity_sprite_size * aspect, entity_sprite_size)
                    }
                    _ => (tile_size, tile_size),
                };
                let offset_x = -(visual_width - tile_size) / 2.0;
                let offset_y = -(visual_height - tile_size) / 2.0;
                entity_node.left = Val::Px(marker.pos.x as f32 * tile_size + offset_x);
                entity_node.top = Val::Px(marker.pos.y as f32 * tile_size + offset_y);
                entity_node.width = Val::Px(visual_width);
                entity_node.height = Val::Px(visual_height);
            }
        }
    }
}
