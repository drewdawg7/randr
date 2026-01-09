use bevy::prelude::*;

use crate::assets::GameSprites;

const CELL_SIZE: f32 = 48.0;
const GRID_SIZE: usize = 5;

/// Plugin for item grid widget.
pub struct ItemGridPlugin;

impl Plugin for ItemGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_add_item_grid);
    }
}

/// Marker for item grid widget. Observer populates with sprite cells.
#[derive(Component)]
pub struct ItemGrid;

fn on_add_item_grid(
    trigger: Trigger<OnAdd, ItemGrid>,
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
) {
    let entity = trigger.entity();

    // Get the cell sprite image if available
    let cell_image = game_sprites.ui_all.as_ref().and_then(|ui_all| {
        ui_all.get("Slice_10").map(|idx| {
            ImageNode::from_atlas_image(
                ui_all.texture.clone(),
                TextureAtlas {
                    layout: ui_all.layout.clone(),
                    index: idx,
                },
            )
        })
    });

    commands
        .entity(entity)
        .insert(Node {
            display: Display::Grid,
            grid_template_columns: RepeatedGridTrack::px(GRID_SIZE as u16, CELL_SIZE),
            grid_template_rows: RepeatedGridTrack::px(GRID_SIZE as u16, CELL_SIZE),
            ..default()
        })
        .with_children(|grid| {
            for _ in 0..(GRID_SIZE * GRID_SIZE) {
                let mut cell = grid.spawn(Node {
                    width: Val::Px(CELL_SIZE),
                    height: Val::Px(CELL_SIZE),
                    ..default()
                });
                if let Some(ref img) = cell_image {
                    cell.insert(img.clone());
                }
            }
        });
}
