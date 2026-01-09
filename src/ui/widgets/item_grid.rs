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

/// An item to display in the grid.
#[derive(Clone)]
pub struct ItemGridEntry {
    /// Slice name in icon_items sprite sheet (e.g., "Slice_337")
    pub sprite_name: &'static str,
}

/// Item grid widget with optional items to display.
#[derive(Component, Default)]
pub struct ItemGrid {
    /// Items to display in the grid cells (up to 25)
    pub items: Vec<ItemGridEntry>,
}

fn on_add_item_grid(
    trigger: Trigger<OnAdd, ItemGrid>,
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    item_grids: Query<&ItemGrid>,
) {
    let entity = trigger.entity();
    let item_grid = item_grids.get(entity).ok();

    // Get the cell background sprite if available
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
            for i in 0..(GRID_SIZE * GRID_SIZE) {
                let mut cell = grid.spawn(Node {
                    width: Val::Px(CELL_SIZE),
                    height: Val::Px(CELL_SIZE),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                });
                if let Some(ref img) = cell_image {
                    cell.insert(img.clone());
                }

                // Add item sprite if there's an item at this index
                if let Some(item_grid) = item_grid {
                    if let Some(entry) = item_grid.items.get(i) {
                        if let Some(icon_items) = &game_sprites.icon_items {
                            if let Some(idx) = icon_items.get(entry.sprite_name) {
                                cell.with_children(|cell_content| {
                                    cell_content.spawn((
                                        Node {
                                            width: Val::Px(32.0),
                                            height: Val::Px(32.0),
                                            ..default()
                                        },
                                        ImageNode::from_atlas_image(
                                            icon_items.texture.clone(),
                                            TextureAtlas {
                                                layout: icon_items.layout.clone(),
                                                index: idx,
                                            },
                                        ),
                                    ));
                                });
                            }
                        }
                    }
                }
            }
        });
}
