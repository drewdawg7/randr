//! Generic three-slice banner spawning helper.

use bevy::prelude::*;

use crate::assets::{GameSprites, ThreeSlice};

/// Spawns a three-slice horizontal banner using a 1x3 CSS grid.
///
/// The banner uses fixed-width left and right edges with a stretchable center.
///
/// # Type Parameters
/// - `S`: A type implementing [`ThreeSlice`] that defines the slices, dimensions, and sprite sheet.
///
/// # Arguments
/// - `parent`: The parent entity's `ChildBuilder`
/// - `game_sprites`: The game sprites resource for sprite lookups
/// - `width`: Total width of the banner
pub fn spawn_three_slice_banner<S: ThreeSlice>(
    parent: &mut ChildBuilder,
    game_sprites: &GameSprites,
    width: f32,
) {
    let Some(sheet) = game_sprites.get(S::SHEET_KEY) else {
        return;
    };

    let stretch_width = width - (S::EDGE_WIDTH * 2.0);

    parent
        .spawn(Node {
            width: Val::Px(width),
            height: Val::Px(S::HEIGHT),
            display: Display::Grid,
            grid_template_columns: vec![
                GridTrack::px(S::EDGE_WIDTH),
                GridTrack::px(stretch_width),
                GridTrack::px(S::EDGE_WIDTH),
            ],
            grid_template_rows: vec![GridTrack::px(S::HEIGHT)],
            ..default()
        })
        .with_children(|grid| {
            for slice in S::ALL {
                let w = slice.width(stretch_width);

                let mut cell = grid.spawn(Node {
                    width: Val::Px(w),
                    height: Val::Px(S::HEIGHT),
                    ..default()
                });

                if let Some(img) = sheet.image_node(slice.as_str()) {
                    cell.insert(img);
                }
            }
        });
}
