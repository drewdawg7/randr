use bevy::prelude::*;

use super::cell::{GridCell, GridCellBundle, GridContainer};
use super::components::{GridItemQuantityText, GridItemSprite, ItemGrid, ItemGridFocusPanel};
use super::{CELL_SIZE, GAP, NINE_SLICE_INSET};
use crate::assets::{GameFonts, GameSprites, GridSlotSlice, ShopBgSlice, SpriteSheetKey};
use crate::ui::focus::FocusState;
use crate::ui::widgets::nine_slice::spawn_nine_slice_panel;
use crate::ui::widgets::outlined_text::{spawn_outlined_quantity_text, OutlinedQuantityConfig};
use crate::ui::widgets::selector::{spawn_selector, AnimatedSelector};

const ITEM_SPRITE_SIZE: f32 = 32.0;

pub fn on_add_item_grid(
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
                                            width: Val::Px(ITEM_SPRITE_SIZE),
                                            height: Val::Px(ITEM_SPRITE_SIZE),
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

pub fn update_grid_items(
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
                                width: Val::Px(ITEM_SPRITE_SIZE),
                                height: Val::Px(ITEM_SPRITE_SIZE),
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

pub fn update_grid_selector(
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
