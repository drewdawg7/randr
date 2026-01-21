use crate::assets::DungeonTileSlice;

use super::{DungeonLayout, TileType};

pub struct TileRenderer;

impl TileRenderer {
    pub fn resolve(layout: &DungeonLayout, x: usize, y: usize) -> Option<(DungeonTileSlice, bool)> {
        let tile = layout.tile_at(x, y)?;

        match tile.tile_type {
            TileType::Floor | TileType::Entrance | TileType::PlayerSpawn => {
                Some(Self::resolve_floor(tile.variant))
            }
            TileType::Wall => Some(Self::resolve_wall(layout, x, y)),
            TileType::Exit => Some((DungeonTileSlice::Gate, false)),
            TileType::Door => Some((DungeonTileSlice::Gate, false)),
            TileType::DoorOpen => Some((DungeonTileSlice::GateFloor, false)),
        }
    }

    fn resolve_floor(variant: u8) -> (DungeonTileSlice, bool) {
        let slice = match variant % 3 {
            0 => DungeonTileSlice::FloorTile2,
            1 => DungeonTileSlice::FloorTile3,
            _ => DungeonTileSlice::FloorTile4,
        };
        (slice, false)
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
