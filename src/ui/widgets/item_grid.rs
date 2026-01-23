use bevy::prelude::*;

use super::nine_slice::spawn_nine_slice_panel;
use crate::assets::{GameSprites, GridSlotSlice, ShopBgSlice, SpriteSheetKey, UiSelectorsSlice};

const CELL_SIZE: f32 = 48.0;
const GAP: f32 = 4.0;
const NINE_SLICE_INSET: f32 = 58.0;

/// Plugin for item grid widget.
pub struct ItemGridPlugin;

impl Plugin for ItemGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_add_item_grid)
            // Run in PostUpdate to avoid race with UI refresh systems in Update
            .add_systems(PostUpdate, (update_grid_items, update_grid_selector, animate_grid_selector).chain());
    }
}

/// An item to display in the grid.
#[derive(Clone)]
pub struct ItemGridEntry {
    /// Slice name in icon_items sprite sheet (e.g., "Slice_337")
    pub sprite_name: String,
}

/// Item grid widget with optional items to display.
#[derive(Component)]
pub struct ItemGrid {
    /// Items to display in the grid cells
    pub items: Vec<ItemGridEntry>,
    /// Currently selected cell index
    pub selected_index: usize,
    /// Whether this grid is focused (shows selector)
    pub is_focused: bool,
    /// Number of columns/rows in the grid (e.g., 3 for 3x3, 4 for 4x4)
    pub grid_size: usize,
}

impl Default for ItemGrid {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            selected_index: 0,
            is_focused: true,
            grid_size: 4,
        }
    }
}

/// Marker for the grid container (child of ItemGrid that holds the cells).
#[derive(Component)]
pub struct GridContainer;

/// Marker for grid cells with their index.
#[derive(Component)]
pub struct GridCell {
    pub index: usize,
}

/// Marker for the selector sprite with animation state.
#[derive(Component)]
pub struct GridSelector {
    /// Timer for animation
    pub timer: Timer,
    /// Current frame (0 = Slice_61, 1 = Slice_91)
    pub frame: usize,
    /// Atlas indices for the two frames
    pub frame_indices: [usize; 2],
}

fn on_add_item_grid(
    trigger: Trigger<OnAdd, ItemGrid>,
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    item_grids: Query<&ItemGrid>,
) {
    let entity = trigger.entity();
    let item_grid = item_grids.get(entity).ok();
    let selected_index = item_grid.map(|g| g.selected_index).unwrap_or(0);
    let is_focused = item_grid.map(|g| g.is_focused).unwrap_or(true);
    let grid_size = item_grid.map(|g| g.grid_size).unwrap_or(4);

    // Get the cell background sprite if available
    let cell_image = game_sprites
        .get(SpriteSheetKey::GridSlot)
        .and_then(|s| s.image_node(GridSlotSlice::Slot.as_str()));

    // Get the selector sprite frames if available
    let selector_data = game_sprites
        .get(SpriteSheetKey::UiSelectors)
        .and_then(|selectors| {
            let idx1 = selectors.get(UiSelectorsSlice::SelectorFrame1.as_str())?;
            let idx2 = selectors.get(UiSelectorsSlice::SelectorFrame2.as_str())?;
            Some((
                selectors.image_node(UiSelectorsSlice::SelectorFrame1.as_str())?,
                [idx1, idx2],
            ))
        });

    let content_size = grid_size as f32 * CELL_SIZE + (grid_size - 1) as f32 * GAP;
    let grid_width = content_size + 2.0 * NINE_SLICE_INSET;
    let grid_height = grid_width;

    // Set up the ItemGrid entity as the container
    let mut item_grid_entity = commands.entity(entity);
    item_grid_entity.insert(Node {
        width: Val::Px(grid_width),
        height: Val::Px(grid_height),
        position_type: PositionType::Relative,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    });

    item_grid_entity.with_children(|parent| {
        // Spawn nine-slice background (absolute positioned behind grid)
        spawn_nine_slice_panel::<ShopBgSlice>(parent, &game_sprites, grid_width, grid_height);

        // Grid container on top
        parent
            .spawn((
                GridContainer,
                Node {
                    display: Display::Grid,
                    grid_template_columns: RepeatedGridTrack::px(grid_size as u16, CELL_SIZE),
                    grid_template_rows: RepeatedGridTrack::px(grid_size as u16, CELL_SIZE),
                    row_gap: Val::Px(GAP),
                    column_gap: Val::Px(GAP),
                    ..default()
                },
            ))
            .with_children(|grid| {
                for i in 0..(grid_size * grid_size) {
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

                    // Add selector sprite if this is the selected cell and grid is focused
                    let is_selected = i == selected_index;
                    if is_selected && is_focused {
                        if let Some((ref selector_img, frame_indices)) = selector_data {
                            cell.with_children(|cell_content| {
                                cell_content.spawn((
                                    GridSelector {
                                        timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                                        frame: 0,
                                        frame_indices,
                                    },
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
                            if let Some(icon_img) = game_sprites
                                .get(SpriteSheetKey::IconItems)
                                .and_then(|s| s.image_node(&entry.sprite_name))
                            {
                                cell.with_children(|cell_content| {
                                    cell_content.spawn((
                                        GridItemSprite,
                                        Node {
                                            width: Val::Px(32.0),
                                            height: Val::Px(32.0),
                                            ..default()
                                        },
                                        icon_img,
                                    ));
                                });
                            }
                        }
                    }
                }
            });
    });
}

/// Marker for item sprites inside grid cells (to distinguish from selector sprites).
#[derive(Component)]
struct GridItemSprite;

/// Update the item sprites in grid cells when the items list changes.
fn update_grid_items(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    item_grids: Query<(&ItemGrid, &Children), Changed<ItemGrid>>,
    grid_containers: Query<&Children, With<GridContainer>>,
    grid_cells: Query<(Entity, &GridCell, Option<&Children>)>,
    item_sprites: Query<Entity, With<GridItemSprite>>,
) {
    for (item_grid, item_grid_children) in &item_grids {
        // Find the GridContainer child
        let Some(container_children) = item_grid_children
            .iter()
            .find_map(|&child| grid_containers.get(child).ok())
        else {
            continue;
        };

        for &child in container_children.iter() {
            let Ok((cell_entity, grid_cell, cell_children)) = grid_cells.get(child) else {
                continue;
            };

            // Remove existing item sprites from this cell
            if let Some(children) = cell_children {
                for &cell_child in children.iter() {
                    if item_sprites.contains(cell_child) {
                        if commands.get_entity(cell_child).is_some() {
                            commands.entity(cell_child).despawn_recursive();
                        }
                    }
                }
            }

            // Add item sprite if there's an item at this cell index
            if let Some(entry) = item_grid.items.get(grid_cell.index) {
                if let Some(icon_img) = game_sprites
                    .get(SpriteSheetKey::IconItems)
                    .and_then(|s| s.image_node(&entry.sprite_name))
                {
                    commands.entity(cell_entity).with_children(|cell_content| {
                        cell_content.spawn((
                            GridItemSprite,
                            Node {
                                width: Val::Px(32.0),
                                height: Val::Px(32.0),
                                ..default()
                            },
                            icon_img,
                        ));
                    });
                }
            }
        }
    }
}

/// Update the grid selector position when selection changes.
fn update_grid_selector(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    item_grids: Query<(Entity, &ItemGrid, &Children), Changed<ItemGrid>>,
    grid_containers: Query<&Children, With<GridContainer>>,
    grid_cells: Query<(Entity, &GridCell, &Children)>,
    selectors: Query<Entity, With<GridSelector>>,
) {
    for (grid_entity, item_grid, item_grid_children) in &item_grids {
        // Skip if the grid entity is being despawned
        if commands.get_entity(grid_entity).is_none() {
            continue;
        }

        // Find the GridContainer child to get the actual grid cells
        let Some(container_children) = item_grid_children
            .iter()
            .find_map(|&child| grid_containers.get(child).ok())
        else {
            continue;
        };

        // Remove existing selector from this grid only (check children of grid cells)
        for &child in container_children.iter() {
            if let Ok((_, _, cell_children)) = grid_cells.get(child) {
                for &cell_child in cell_children.iter() {
                    if selectors.contains(cell_child) {
                        if commands.get_entity(cell_child).is_some() {
                            commands.entity(cell_child).despawn_recursive();
                        }
                    }
                }
            }
        }

        // Only add selector if grid is focused
        if !item_grid.is_focused {
            continue;
        }

        // Find the selected cell and add selector
        for &child in container_children.iter() {
            if let Ok((cell_entity, grid_cell, _)) = grid_cells.get(child) {
                if grid_cell.index == item_grid.selected_index {
                    // Skip if cell entity no longer exists
                    if commands.get_entity(cell_entity).is_none() {
                        break;
                    }

                    // Add selector to this cell
                    if let Some(selectors_sheet) = game_sprites.get(SpriteSheetKey::UiSelectors) {
                        if let (Some(idx1), Some(idx2), Some(img)) = (
                            selectors_sheet.get(UiSelectorsSlice::SelectorFrame1.as_str()),
                            selectors_sheet.get(UiSelectorsSlice::SelectorFrame2.as_str()),
                            selectors_sheet.image_node(UiSelectorsSlice::SelectorFrame1.as_str()),
                        ) {
                            commands.entity(cell_entity).with_children(|cell| {
                                cell.spawn((
                                    GridSelector {
                                        timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                                        frame: 0,
                                        frame_indices: [idx1, idx2],
                                    },
                                    Node {
                                        position_type: PositionType::Absolute,
                                        width: Val::Px(CELL_SIZE),
                                        height: Val::Px(CELL_SIZE),
                                        ..default()
                                    },
                                    img,
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

/// Animate the grid selector by alternating between frames.
fn animate_grid_selector(
    time: Res<Time>,
    mut selectors: Query<(&mut GridSelector, &mut ImageNode)>,
) {
    for (mut selector, mut image) in &mut selectors {
        selector.timer.tick(time.delta());
        if selector.timer.just_finished() {
            // Toggle frame
            selector.frame = (selector.frame + 1) % 2;
            // Update the atlas index
            if let Some(ref mut atlas) = image.texture_atlas {
                atlas.index = selector.frame_indices[selector.frame];
            }
        }
    }
}
