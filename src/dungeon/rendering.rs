use crate::assets::{CaveTileSlice, DungeonTileSlice, SpriteSheetKey};

use super::{DungeonLayout, FloorType, TileType};

pub struct ResolvedTile {
    pub slice_name: &'static str,
    pub flip_x: bool,
    pub tileset_key: SpriteSheetKey,
}

pub fn resolve_tile(
    _floor_type: FloorType,
    layout: &DungeonLayout,
    x: usize,
    y: usize,
) -> Option<ResolvedTile> {
    CaveTileRenderer::resolve(layout, x, y).map(|result| {
        let tileset_key = if result.uses_dungeon_tileset {
            SpriteSheetKey::DungeonTileset
        } else {
            SpriteSheetKey::CaveTileset
        };
        ResolvedTile {
            slice_name: result.slice_name,
            flip_x: result.flip_x,
            tileset_key,
        }
    })
}

pub struct CaveTileRenderer;

pub struct CaveTileResult {
    pub slice_name: &'static str,
    pub flip_x: bool,
    pub uses_dungeon_tileset: bool,
}

impl CaveTileRenderer {
    pub fn resolve(layout: &DungeonLayout, x: usize, y: usize) -> Option<CaveTileResult> {
        let tile = layout.tile_at(x, y)?;

        match tile.tile_type {
            TileType::PlayerSpawn => Some(CaveTileResult {
                slice_name: DungeonTileSlice::GateFloor.as_str(),
                flip_x: false,
                uses_dungeon_tileset: true,
            }),
            TileType::Floor | TileType::Entrance | TileType::SpawnPoint => {
                Some(Self::resolve_floor(tile.variant))
            }
            TileType::Wall => Self::resolve_wall(layout, x, y),
            TileType::Exit => Some(CaveTileResult {
                slice_name: DungeonTileSlice::Gate.as_str(),
                flip_x: false,
                uses_dungeon_tileset: true,
            }),
            TileType::Door => Some(CaveTileResult {
                slice_name: DungeonTileSlice::Gate.as_str(),
                flip_x: false,
                uses_dungeon_tileset: true,
            }),
            TileType::DoorOpen => Some(CaveTileResult {
                slice_name: DungeonTileSlice::GateFloor.as_str(),
                flip_x: false,
                uses_dungeon_tileset: true,
            }),
            TileType::TorchWall => Self::resolve_wall(layout, x, y),
            TileType::Empty => None,
        }
    }

    fn resolve_floor(variant: u8) -> CaveTileResult {
        let slice = match variant % 6 {
            0 => CaveTileSlice::Floor1,
            1 => CaveTileSlice::Floor2,
            2 => CaveTileSlice::Floor3,
            3 => CaveTileSlice::Floor4,
            4 => CaveTileSlice::Floor5,
            _ => CaveTileSlice::Floor6,
        };
        CaveTileResult {
            slice_name: slice.as_str(),
            flip_x: false,
            uses_dungeon_tileset: false,
        }
    }

    fn resolve_wall(layout: &DungeonLayout, x: usize, y: usize) -> Option<CaveTileResult> {
        if x == 0 {
            return Some(CaveTileResult {
                slice_name: CaveTileSlice::RightRoof.as_str(),
                flip_x: false,
                uses_dungeon_tileset: false,
            });
        }
        if x == layout.width() - 1 {
            return Some(CaveTileResult {
                slice_name: CaveTileSlice::LeftRoof.as_str(),
                flip_x: false,
                uses_dungeon_tileset: false,
            });
        }

        let ctx = WallContext::analyze(layout, x, y);

        if ctx.floor_above && !ctx.floor_below {
            return Some(CaveTileResult {
                slice_name: CaveTileSlice::FrontRoof.as_str(),
                flip_x: false,
                uses_dungeon_tileset: false,
            });
        }

        None
    }
}

struct WallContext {
    floor_above: bool,
    floor_below: bool,
}

impl WallContext {
    fn analyze(layout: &DungeonLayout, x: usize, y: usize) -> Self {
        let is_floor = |dx: i32, dy: i32| -> bool {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || ny < 0 {
                return false;
            }
            layout.is_floor(nx as usize, ny as usize)
        };

        Self {
            floor_above: is_floor(0, -1),
            floor_below: is_floor(0, 1),
        }
    }
}
