//! Generic nine-slice panel spawning helper.

use bevy::prelude::*;

use crate::assets::{GameSprites, NineSlice};

/// Spawns a nine-slice panel background using a 3x3 CSS grid.
///
/// The panel is positioned absolutely at (0, 0) within its parent.
/// Corner slices remain fixed size while edge and center slices stretch.
///
/// # Type Parameters
/// - `S`: A type implementing [`NineSlice`] that defines the slices, size, and sprite sheet.
///
/// # Arguments
/// - `parent`: The parent entity's `ChildBuilder`
/// - `game_sprites`: The game sprites resource for sprite lookups
/// - `width`: Total width of the panel
/// - `height`: Total height of the panel
pub fn spawn_nine_slice_panel<S: NineSlice>(
    parent: &mut ChildBuilder,
    game_sprites: &GameSprites,
    width: f32,
    height: f32,
) {
    let Some(sheet) = game_sprites.get(S::SHEET_KEY) else {
        return;
    };

    let stretch_width = width - (S::SLICE_SIZE * 2.0);
    let stretch_height = height - (S::SLICE_SIZE * 2.0);

    parent
        .spawn(Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            width: Val::Px(width),
            height: Val::Px(height),
            display: Display::Grid,
            grid_template_columns: vec![
                GridTrack::px(S::SLICE_SIZE),
                GridTrack::px(stretch_width),
                GridTrack::px(S::SLICE_SIZE),
            ],
            grid_template_rows: vec![
                GridTrack::px(S::SLICE_SIZE),
                GridTrack::px(stretch_height),
                GridTrack::px(S::SLICE_SIZE),
            ],
            ..default()
        })
        .with_children(|grid| {
            for slice in S::ALL {
                let (w, h) = slice.dimensions(stretch_width, stretch_height);

                let mut cell = grid.spawn(Node {
                    width: Val::Px(w),
                    height: Val::Px(h),
                    ..default()
                });

                if let Some(img) = sheet.image_node(slice.as_str()) {
                    cell.insert(img);
                }
            }
        });
}
