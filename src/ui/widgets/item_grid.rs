use bevy::prelude::*;

use crate::assets::GameSprites;

const CELL_SIZE: f32 = 48.0;
const GRID_SIZE: usize = 5;

/// Plugin for item grid widget.
pub struct ItemGridPlugin;

impl Plugin for ItemGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_add_item_grid)
            .add_systems(Update, update_grid_selector);
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
    /// Currently selected cell index
    pub selected_index: usize,
}

/// Marker for grid cells with their index.
#[derive(Component)]
pub struct GridCell {
    pub index: usize,
}

/// Marker for the selector sprite.
#[derive(Component)]
pub struct GridSelector;

fn on_add_item_grid(
    trigger: Trigger<OnAdd, ItemGrid>,
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    item_grids: Query<&ItemGrid>,
) {
    let entity = trigger.entity();
    let item_grid = item_grids.get(entity).ok();
    let selected_index = item_grid.map(|g| g.selected_index).unwrap_or(0);

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

    // Get the selector sprite if available
    let selector_image = game_sprites.ui_selectors.as_ref().and_then(|selectors| {
        selectors.get("Slice_61").map(|idx| {
            ImageNode::from_atlas_image(
                selectors.texture.clone(),
                TextureAtlas {
                    layout: selectors.layout.clone(),
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
                let mut cell = grid.spawn((
                    GridCell { index: i },
                    Node {
                        width: Val::Px(CELL_SIZE),
                        height: Val::Px(CELL_SIZE),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ));
                if let Some(ref img) = cell_image {
                    cell.insert(img.clone());
                }

                // Add selector sprite if this is the selected cell
                let is_selected = i == selected_index;
                if is_selected {
                    if let Some(ref selector_img) = selector_image {
                        cell.with_children(|cell_content| {
                            cell_content.spawn((
                                GridSelector,
                                Node {
                                    position_type: PositionType::Absolute,
                                    width: Val::Px(CELL_SIZE),
                                    height: Val::Px(CELL_SIZE),
                                    ..default()
                                },
                                selector_img.clone(),
                            ));
                        });
                    }
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

/// Update the grid selector position when selection changes.
fn update_grid_selector(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    item_grids: Query<(&ItemGrid, &Children), Changed<ItemGrid>>,
    grid_cells: Query<(Entity, &GridCell, &Children)>,
    selectors: Query<Entity, With<GridSelector>>,
) {
    for (item_grid, grid_children) in &item_grids {
        // Remove existing selector
        for selector_entity in &selectors {
            commands.entity(selector_entity).despawn_recursive();
        }

        // Find the selected cell and add selector
        for &child in grid_children.iter() {
            if let Ok((cell_entity, grid_cell, _)) = grid_cells.get(child) {
                if grid_cell.index == item_grid.selected_index {
                    // Add selector to this cell
                    if let Some(selectors_sheet) = &game_sprites.ui_selectors {
                        if let Some(idx) = selectors_sheet.get("Slice_61") {
                            commands.entity(cell_entity).with_children(|cell| {
                                cell.spawn((
                                    GridSelector,
                                    Node {
                                        position_type: PositionType::Absolute,
                                        width: Val::Px(CELL_SIZE),
                                        height: Val::Px(CELL_SIZE),
                                        ..default()
                                    },
                                    ImageNode::from_atlas_image(
                                        selectors_sheet.texture.clone(),
                                        TextureAtlas {
                                            layout: selectors_sheet.layout.clone(),
                                            index: idx,
                                        },
                                    ),
                                ));
                            });
                        }
                    }
                    break;
                }
            }
        }
    }
}
