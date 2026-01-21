//! Generic three-slice banner spawning helper.

use bevy::prelude::*;

use crate::assets::{GameSprites, ThreeSlice};
use crate::ui::text::UiText;

/// Spawns a three-slice horizontal banner using a 1x3 CSS grid.
///
/// The banner uses fixed-width left and right edges with a stretchable center.
/// Optionally displays centered text overlaid on the banner.
///
/// # Type Parameters
/// - `S`: A type implementing [`ThreeSlice`] that defines the slices, dimensions, and sprite sheet.
///
/// # Arguments
/// - `parent`: The parent entity's `ChildBuilder`
/// - `game_sprites`: The game sprites resource for sprite lookups
/// - `width`: Total width of the banner
/// - `text`: Optional text to display centered on the banner
pub fn spawn_three_slice_banner<S: ThreeSlice>(
    parent: &mut ChildBuilder,
    game_sprites: &GameSprites,
    width: f32,
    text: Option<&str>,
) {
    let Some(sheet) = game_sprites.get(S::SHEET_KEY) else {
        return;
    };

    let stretch_width = width - (S::EDGE_WIDTH * 2.0);

    // Wrapper to allow absolute positioning of text over the grid
    parent
        .spawn(Node {
            width: Val::Px(width),
            height: Val::Px(S::HEIGHT),
            ..default()
        })
        .with_children(|wrapper| {
            // The 3-slice grid
            wrapper
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

            // Text overlay (centered)
            if let Some(label) = text {
                wrapper
                    .spawn(Node {
                        width: Val::Px(width),
                        height: Val::Px(S::HEIGHT),
                        position_type: PositionType::Absolute,
                        top: Val::Px(0.0),
                        left: Val::Px(0.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    })
                    .with_children(|text_container| {
                        text_container.spawn(UiText::new(label).size(14.0).build());
                    });
            }
        });
}
