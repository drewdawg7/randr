use crate::assets::{CaveTileSlice, DungeonTileSlice, SpriteSheetKey};

use super::{DungeonLayout, FloorType, TileType};

/// Unified tile resolution result that works across different tilesets
pub struct ResolvedTile {
    pub slice_name: &'static str,
    pub flip_x: bool,
    pub tileset_key: SpriteSheetKey,
}

/// Resolves tiles based on floor type, dispatching to the appropriate renderer
pub fn resolve_tile(
    floor_type: FloorType,
    layout: &DungeonLayout,
    x: usize,
    y: usize,
) -> Option<ResolvedTile> {
    match floor_type {
        FloorType::BasicDungeonFloor => {
            TileRenderer::resolve(layout, x, y).map(|(slice, flip_x)| ResolvedTile {
                slice_name: slice.as_str(),
                flip_x,
                tileset_key: SpriteSheetKey::DungeonTileset,
            })
        }
        FloorType::CaveFloor | FloorType::TmxCaveFloor => {
            CaveTileRenderer::resolve(layout, x, y).map(|result| {
                // Gate/GateFloor/Stairs come from dungeon tileset
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
    }
}

pub struct TileRenderer;

impl TileRenderer {
    pub fn resolve(layout: &DungeonLayout, x: usize, y: usize) -> Option<(DungeonTileSlice, bool)> {
        let tile = layout.tile_at(x, y)?;

        match tile.tile_type {
            TileType::PlayerSpawn => Some((DungeonTileSlice::GateFloor, false)),
            TileType::Floor | TileType::Entrance | TileType::SpawnPoint => {
                if let Some(edge) = Self::resolve_floor_edge(layout, x, y) {
                    Some(edge)
                } else {
                    Some(Self::resolve_floor(tile.variant))
                }
            }
            TileType::Wall => Some(Self::resolve_wall(layout, x, y)),
            TileType::Exit => Some((DungeonTileSlice::Gate, false)),
            TileType::Door => Some((DungeonTileSlice::Gate, false)),
            TileType::DoorOpen => Some((DungeonTileSlice::GateFloor, false)),
            TileType::TorchWall | TileType::Empty => None,
        }
    }

    fn resolve_floor(variant: u8) -> (DungeonTileSlice, bool) {
        let slice = match variant % 5 {
            0 => DungeonTileSlice::FloorTileAlt1,
            1 => DungeonTileSlice::FloorTile2,
            2 => DungeonTileSlice::FloorTile3,
            3 => DungeonTileSlice::FloorTile4,
            _ => DungeonTileSlice::FloorTileAlt3,
        };
        (slice, false)
    }

    fn resolve_floor_edge(
        layout: &DungeonLayout,
        x: usize,
        y: usize,
    ) -> Option<(DungeonTileSlice, bool)> {
        let wall_above = y == 0 || !layout.is_floor(x, y - 1);
        let wall_left = x == 0 || !layout.is_floor(x - 1, y);
        let wall_right = x >= layout.width() - 1 || !layout.is_floor(x + 1, y);

        // Corners first
        if wall_above && wall_left {
            return Some((DungeonTileSlice::FloorEdgeTopLeft, false));
        }
        if wall_above && wall_right {
            return Some((DungeonTileSlice::FloorEdgeTopRight, false));
        }

        // Top edge
        if wall_above {
            let slice = if x.is_multiple_of(2) {
                DungeonTileSlice::FloorEdgeTop1
            } else {
                DungeonTileSlice::FloorEdgeTop2
            };
            return Some((slice, false));
        }

        // Left edge
        if wall_left {
            let wall_below = y >= layout.height() - 1 || !layout.is_floor(x, y + 1);
            let slice = if wall_below {
                DungeonTileSlice::FloorEdgeLeft2
            } else {
                DungeonTileSlice::FloorEdgeLeft
            };
            return Some((slice, false));
        }

        // Right edge
        if wall_right {
            let wall_below = y >= layout.height() - 1 || !layout.is_floor(x, y + 1);
            let slice = if wall_below {
                DungeonTileSlice::FloorEdgeRight2
            } else {
                DungeonTileSlice::FloorEdgeRight1
            };
            return Some((slice, false));
        }

        // Bottom edge (inner tiles only, not corners handled by left/right edges)
        let wall_below = y >= layout.height() - 1 || !layout.is_floor(x, y + 1);
        if wall_below {
            let slice = if x.is_multiple_of(2) {
                DungeonTileSlice::FloorTileAlt2
            } else {
                DungeonTileSlice::FloorTileAlt4
            };
            return Some((slice, false));
        }

        None
    }

    fn resolve_wall(layout: &DungeonLayout, x: usize, y: usize) -> (DungeonTileSlice, bool) {
        let ctx = WallContext::analyze(layout, x, y);

        // Corner detection uses diagonal floor check
        // Top-left corner: walls to right and below, floor diagonally
        if ctx.wall_right && ctx.wall_below && ctx.floor_diag_br {
            return (DungeonTileSlice::SideWall5, true);
        }
        // Top-right corner: walls to left and below, floor diagonally
        if ctx.wall_left && ctx.wall_below && ctx.floor_diag_bl {
            return (DungeonTileSlice::SideWall5, false);
        }
        // Bottom-left corner: walls to right and above, floor diagonally
        if ctx.wall_right && ctx.wall_above && ctx.floor_diag_tr {
            return (DungeonTileSlice::BottomRightWall, true);
        }
        // Bottom-right corner: walls to left and above, floor diagonally
        if ctx.wall_left && ctx.wall_above && ctx.floor_diag_tl {
            return (DungeonTileSlice::BottomRightWall, false);
        }

        // Top wall (floor below)
        if ctx.floor_below && !ctx.floor_above {
            let variant = (x % 4) as u8;
            let slice = match variant {
                0 => DungeonTileSlice::TopWall1,
                1 => DungeonTileSlice::TopWall2,
                2 => DungeonTileSlice::TopWall3,
                _ => DungeonTileSlice::TopWall4,
            };
            return (slice, false);
        }

        // Bottom wall (floor above)
        if ctx.floor_above && !ctx.floor_below {
            let variant = (x % 4) as u8;
            let slice = match variant {
                0 => DungeonTileSlice::BottomWall1,
                1 => DungeonTileSlice::BottomWall2,
                2 => DungeonTileSlice::BottomWall3,
                _ => DungeonTileSlice::BottomWall4,
            };
            return (slice, false);
        }

        // Left wall (floor to right)
        if ctx.floor_right && !ctx.floor_left {
            let variant = (y % 3) as u8;
            let slice = match variant {
                0 => DungeonTileSlice::SideWall6,
                1 => DungeonTileSlice::SideWall7,
                _ => DungeonTileSlice::SideWall8,
            };
            return (slice, true);
        }

        // Right wall (floor to left)
        if ctx.floor_left && !ctx.floor_right {
            let variant = (y % 3) as u8;
            let slice = match variant {
                0 => DungeonTileSlice::SideWall6,
                1 => DungeonTileSlice::SideWall7,
                _ => DungeonTileSlice::SideWall8,
            };
            return (slice, false);
        }

        (DungeonTileSlice::TopWall1, false)
    }
}

/// Cave-specific tile renderer
pub struct CaveTileRenderer;

/// Result from cave tile resolution
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
            // TorchWall renders as wall in caves (no torches)
            TileType::TorchWall => Self::resolve_wall(layout, x, y),
            // Empty tiles are not rendered
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
        // Edge walls: same tile all the way down each side (checked FIRST)
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

        // Bottom wall (floor above) - front_roof in cave
        if ctx.floor_above && !ctx.floor_below {
            return Some(CaveTileResult {
                slice_name: CaveTileSlice::FrontRoof.as_str(),
                flip_x: false,
                uses_dungeon_tileset: false,
            });
        }

        // No matching wall pattern - don't render
        None
    }
}

struct WallContext {
    floor_above: bool,
    floor_below: bool,
    floor_left: bool,
    floor_right: bool,
    wall_above: bool,
    wall_below: bool,
    wall_left: bool,
    wall_right: bool,
    floor_diag_tl: bool,
    floor_diag_tr: bool,
    floor_diag_bl: bool,
    floor_diag_br: bool,
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

        let is_wall = |dx: i32, dy: i32| -> bool {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || ny < 0 {
                return true; // Treat out of bounds as wall
            }
            layout
                .tile_at(nx as usize, ny as usize)
                .map(|t| t.tile_type == TileType::Wall)
                .unwrap_or(true)
        };

        Self {
            floor_above: is_floor(0, -1),
            floor_below: is_floor(0, 1),
            floor_left: is_floor(-1, 0),
            floor_right: is_floor(1, 0),
            wall_above: is_wall(0, -1),
            wall_below: is_wall(0, 1),
            wall_left: is_wall(-1, 0),
            wall_right: is_wall(1, 0),
            floor_diag_tl: is_floor(-1, -1),
            floor_diag_tr: is_floor(1, -1),
            floor_diag_bl: is_floor(-1, 1),
            floor_diag_br: is_floor(1, 1),
        }
    }
}
