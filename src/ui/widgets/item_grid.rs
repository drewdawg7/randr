use bevy::prelude::*;

use super::nine_slice::spawn_nine_slice_panel;
use super::outlined_text::{spawn_outlined_quantity_text, OutlinedQuantityConfig};
use super::selector::{spawn_selector, AnimatedSelector};
use crate::assets::{GameFonts, GameSprites, GridSlotSlice, ShopBgSlice, SpriteSheetKey};
use crate::input::NavigationDirection;
use crate::inventory::{Inventory, InventoryItem, ManagesItems};
use crate::ui::focus::{FocusPanel, FocusState};

const CELL_SIZE: f32 = 48.0;
const GAP: f32 = 4.0;
const NINE_SLICE_INSET: f32 = 58.0;

pub struct ItemGridPlugin;

impl Plugin for ItemGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_add_item_grid).add_systems(
            PostUpdate,
            (
                update_grid_items,
                update_grid_selector.run_if(
                    resource_exists::<FocusState>
                        .and(resource_changed::<FocusState>)
                        .or(any_match_filter::<Changed<ItemGrid>>),
                ),
            )
                .chain(),
        );
    }
}

#[derive(Clone)]
pub struct ItemGridEntry {
    pub sprite_sheet_key: SpriteSheetKey,
    pub sprite_name: String,
    pub quantity: u32,
}

impl ItemGridEntry {
    pub fn from_inventory_item(inv_item: &InventoryItem) -> Self {
        Self {
            sprite_sheet_key: inv_item.item.item_id.sprite_sheet_key(),
            sprite_name: inv_item.item.item_id.sprite_name().to_string(),
            quantity: inv_item.quantity,
        }
    }

    pub fn from_inventory(inventory: &Inventory) -> Vec<Self> {
        inventory
            .get_inventory_items()
            .iter()
            .map(Self::from_inventory_item)
            .collect()
    }
}

#[derive(Component)]
pub struct ItemGrid {
    pub items: Vec<ItemGridEntry>,
    pub selected_index: usize,
    pub grid_size: usize,
}

impl Default for ItemGrid {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            selected_index: 0,
            grid_size: 4,
        }
    }
}

#[derive(Component)]
pub struct ItemGridFocusPanel(pub FocusPanel);

impl ItemGrid {
    pub fn clamp_selection(&mut self) {
        if self.items.is_empty() {
            self.selected_index = 0;
        } else {
            self.selected_index = self.selected_index.min(self.items.len() - 1);
        }
    }

    pub fn navigate(&mut self, direction: NavigationDirection) {
        let gs = self.grid_size;
        let total_slots = gs * gs;

        let current = self.selected_index;
        let row = current / gs;
        let col = current % gs;

        let new_index = match direction {
            NavigationDirection::Left if col > 0 => current - 1,
            NavigationDirection::Right if col < gs - 1 => current + 1,
            NavigationDirection::Up if row > 0 => current - gs,
            NavigationDirection::Down if row < gs - 1 => current + gs,
            _ => current,
        };

        if new_index < total_slots {
            self.selected_index = new_index;
        }
    }
}

#[derive(Component)]
pub struct GridContainer;

#[derive(Component)]
pub struct GridCell {
    pub index: usize,
}

#[derive(Bundle)]
pub struct GridCellBundle {
    pub cell: GridCell,
    pub node: Node,
}

impl GridCellBundle {
    pub fn new(index: usize) -> Self {
        Self {
            cell: GridCell { index },
            node: Node {
                width: Val::Px(CELL_SIZE),
                height: Val::Px(CELL_SIZE),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        }
    }
}


fn on_add_item_grid(
    trigger: On<Add, ItemGrid>,
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    game_fonts: Res<GameFonts>,
    focus_state: Option<Res<FocusState>>,
    item_grids: Query<(&ItemGrid, Option<&ItemGridFocusPanel>)>,
) {
    let entity = trigger.entity;
    let (item_grid, focus_panel) = item_grids.get(entity).ok().unzip();
    let selected_index = item_grid.map(|g| g.selected_index).unwrap_or(0);
    let grid_size = item_grid.map(|g| g.grid_size).unwrap_or(4);

    let is_focused = focus_panel
        .flatten()
        .zip(focus_state.as_ref())
        .map(|(panel, state)| state.is_focused(panel.0))
        .unwrap_or(false);

    let cell_image = game_sprites
        .get(SpriteSheetKey::GridSlot)
        .and_then(|s| s.image_node(GridSlotSlice::Slot.as_str()));

    let content_size = grid_size as f32 * CELL_SIZE + (grid_size - 1) as f32 * GAP;
    let grid_width = content_size + 2.0 * NINE_SLICE_INSET;
    let grid_height = grid_width;

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
        spawn_nine_slice_panel::<ShopBgSlice>(parent, &game_sprites, grid_width, grid_height);

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
                    let mut cell = grid.spawn(GridCellBundle::new(i));
                    if let Some(ref img) = cell_image {
                        cell.insert(img.clone());
                    }

                    if i == selected_index && is_focused {
                        cell.with_children(|cell_content| {
                            spawn_selector(cell_content, &game_sprites);
                        });
                    }

                    if let Some(item_grid) = item_grid {
                        if let Some(entry) = item_grid.items.get(i) {
                            if let Some(icon_img) = game_sprites
                                .get(entry.sprite_sheet_key)
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

                                    if entry.quantity > 1 {
                                        spawn_outlined_quantity_text(
                                            cell_content,
                                            &game_fonts,
                                            entry.quantity,
                                            OutlinedQuantityConfig::default(),
                                            GridItemQuantityText,
                                        );
                                    }
                                });
                            }
                        }
                    }
                }
            });
    });
}

#[derive(Component)]
struct GridItemSprite;

#[derive(Component)]
struct GridItemQuantityText;

fn update_grid_items(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    game_fonts: Res<GameFonts>,
    item_grids: Query<(&ItemGrid, &Children), Changed<ItemGrid>>,
    grid_containers: Query<&Children, With<GridContainer>>,
    grid_cells: Query<(Entity, &GridCell, Option<&Children>)>,
    item_sprites: Query<Entity, With<GridItemSprite>>,
    quantity_texts: Query<Entity, With<GridItemQuantityText>>,
) {
    for (item_grid, item_grid_children) in &item_grids {
        let Some(container_children) = item_grid_children
            .iter()
            .find_map(|child| grid_containers.get(child).ok())
        else {
            continue;
        };

        for child in container_children.iter() {
            let Ok((cell_entity, grid_cell, cell_children)) = grid_cells.get(child) else {
                continue;
            };

            if let Some(children) = cell_children {
                for cell_child in children.iter() {
                    if item_sprites.contains(cell_child) || quantity_texts.contains(cell_child) {
                        if commands.get_entity(cell_child).is_ok() {
                            commands.entity(cell_child).despawn();
                        }
                    }
                }
            }

            if let Some(entry) = item_grid.items.get(grid_cell.index) {
                if let Some(icon_img) = game_sprites
                    .get(entry.sprite_sheet_key)
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

                        if entry.quantity > 1 {
                            spawn_outlined_quantity_text(
                                cell_content,
                                &game_fonts,
                                entry.quantity,
                                OutlinedQuantityConfig::default(),
                                GridItemQuantityText,
                            );
                        }
                    });
                }
            }
        }
    }
}

fn update_grid_selector(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    focus_state: Option<Res<FocusState>>,
    item_grids: Query<(Entity, &ItemGrid, Option<&ItemGridFocusPanel>, &Children)>,
    grid_containers: Query<&Children, With<GridContainer>>,
    grid_cells: Query<(Entity, &GridCell, Option<&Children>)>,
    selectors: Query<Entity, With<AnimatedSelector>>,
) {
    for (grid_entity, item_grid, focus_panel, item_grid_children) in &item_grids {
        let is_focused = focus_panel
            .zip(focus_state.as_ref())
            .map(|(panel, state)| state.is_focused(panel.0))
            .unwrap_or(false);

        if commands.get_entity(grid_entity).is_err() {
            continue;
        }

        let Some(container_children) = item_grid_children
            .iter()
            .find_map(|child| grid_containers.get(child).ok())
        else {
            continue;
        };

        for child in container_children.iter() {
            if let Ok((_, _, Some(cell_children))) = grid_cells.get(child) {
                for cell_child in cell_children.iter() {
                    if selectors.contains(cell_child) {
                        if commands.get_entity(cell_child).is_ok() {
                            commands.entity(cell_child).despawn();
                        }
                    }
                }
            }
        }

        if !is_focused {
            continue;
        }

        for child in container_children.iter() {
            if let Ok((cell_entity, grid_cell, _)) = grid_cells.get(child) {
                if grid_cell.index == item_grid.selected_index {
                    if commands.get_entity(cell_entity).is_err() {
                        break;
                    }

                    commands.entity(cell_entity).with_children(|cell| {
                        spawn_selector(cell, &game_sprites);
                    });
                    break;
                }
            }
        }
    }
}

