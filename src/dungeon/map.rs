use bevy_ecs_tiled::prelude::*;

use super::grid::GridPosition;
use super::layout::DungeonLayout;
use super::tile::{Tile, TileType};

pub fn map_to_layout(map: &tiled::Map) -> DungeonLayout {
    let width = map.width as usize;
    let height = map.height as usize;

    let mut layout = DungeonLayout::new(width, height);
    let mut spawn_candidates: Vec<(usize, usize)> = Vec::new();

    let Some(tile_layer) = map.layers().find_map(|layer| layer.as_tile_layer()) else {
        return layout;
    };

    for y in 0..height {
        for x in 0..width {
            let Some(layer_tile) = tile_layer.get_tile(x as i32, y as i32) else {
                layout.set_tile(x, y, Tile::new(TileType::Empty));
                continue;
            };

            let tile_data = layer_tile.get_tile();
            let gid = layer_tile.id();

            let (tile_type, can_spawn_player) = if let Some(data) = tile_data {
                let is_solid = get_bool_property(&data.properties, "is_solid").unwrap_or(true);
                let is_door = get_bool_property(&data.properties, "is_door").unwrap_or(false);
                let can_spawn = get_bool_property(&data.properties, "can_spawn_player").unwrap_or(false);

                let tile_type = if is_door {
                    TileType::Door
                } else if is_solid {
                    TileType::Wall
                } else {
                    TileType::Floor
                };

                (tile_type, can_spawn)
            } else {
                (TileType::Wall, false)
            };

            if can_spawn_player {
                spawn_candidates.push((x, y));
            }

            let tile = Tile::new(tile_type).with_tileset_id(gid + 1);
            layout.set_tile(x, y, tile);
        }
    }

    if let Some(&(x, y)) = spawn_candidates.first() {
        layout.entrance = (x, y);
        if let Some(existing) = layout.tile_at(x, y) {
            let tileset_id = existing.tileset_id;
            let mut tile = Tile::new(TileType::SpawnPoint);
            if let Some(id) = tileset_id {
                tile = tile.with_tileset_id(id);
            }
            layout.set_tile(x, y, tile);
        }
    } else {
        layout.entrance = (width / 2, height / 2);
    }

    layout
}

pub fn entity_spawn_positions(map: &tiled::Map) -> Vec<GridPosition> {
    let width = map.width as usize;
    let height = map.height as usize;
    let mut positions = Vec::new();

    let Some(tile_layer) = map.layers().find_map(|layer| layer.as_tile_layer()) else {
        return positions;
    };

    for y in 0..height {
        for x in 0..width {
            let Some(layer_tile) = tile_layer.get_tile(x as i32, y as i32) else {
                continue;
            };

            if let Some(data) = layer_tile.get_tile() {
                let is_solid = get_bool_property(&data.properties, "is_solid").unwrap_or(true);
                let can_entity = get_bool_property(&data.properties, "can_have_entity").unwrap_or(false);

                if can_entity && !is_solid {
                    positions.push(GridPosition::new(x, y));
                }
            }
        }
    }

    positions
}

pub fn player_spawn_positions(map: &tiled::Map) -> Vec<GridPosition> {
    let width = map.width as usize;
    let height = map.height as usize;
    let mut positions = Vec::new();

    let Some(tile_layer) = map.layers().find_map(|layer| layer.as_tile_layer()) else {
        return positions;
    };

    for y in 0..height {
        for x in 0..width {
            let Some(layer_tile) = tile_layer.get_tile(x as i32, y as i32) else {
                continue;
            };

            if let Some(data) = layer_tile.get_tile() {
                let is_solid = get_bool_property(&data.properties, "is_solid").unwrap_or(true);
                let can_spawn = get_bool_property(&data.properties, "can_spawn_player").unwrap_or(false);

                if can_spawn && !is_solid {
                    positions.push(GridPosition::new(x, y));
                }
            }
        }
    }

    positions
}

fn get_bool_property(properties: &tiled::Properties, name: &str) -> Option<bool> {
    properties.get(name).and_then(|v| {
        if let tiled::PropertyValue::BoolValue(b) = v {
            Some(*b)
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_cave_floor() {
        let mut loader = tiled::Loader::new();
        let map = loader.load_tmx_map("assets/maps/cave_floor.tmx").expect("Failed to load map");

        let layout = map_to_layout(&map);
        assert_eq!(layout.width(), 15);
        assert_eq!(layout.height(), 11);
    }

    #[test]
    fn test_entity_spawn_positions() {
        let mut loader = tiled::Loader::new();
        let map = loader.load_tmx_map("assets/maps/cave_floor.tmx").expect("Failed to load map");

        let positions = entity_spawn_positions(&map);
        assert!(!positions.is_empty());
    }
}
