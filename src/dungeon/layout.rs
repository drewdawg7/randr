use super::tile::{Tile, TileType};

#[derive(Debug, Clone)]
pub struct DungeonLayout {
    width: usize,
    height: usize,
    tiles: Vec<Vec<Tile>>,
    pub entrance: (usize, usize),
    pub exit: Option<(usize, usize)>,
}

impl DungeonLayout {
    pub fn new(width: usize, height: usize) -> Self {
        let tiles = vec![vec![Tile::default(); width]; height];
        Self {
            width,
            height,
            tiles,
            entrance: (0, 0),
            exit: None,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn tile_at(&self, x: usize, y: usize) -> Option<&Tile> {
        self.tiles.get(y).and_then(|row| row.get(x))
    }

    pub fn tile_at_mut(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
        self.tiles.get_mut(y).and_then(|row| row.get_mut(x))
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        if let Some(t) = self.tile_at_mut(x, y) {
            *t = tile;
        }
    }

    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        self.tile_at(x, y)
            .map(|t| t.tile_type.is_walkable())
            .unwrap_or(false)
    }

    pub fn is_floor(&self, x: usize, y: usize) -> bool {
        self.tile_at(x, y)
            .map(|t| {
                matches!(
                    t.tile_type,
                    TileType::Floor | TileType::Entrance | TileType::Exit | TileType::DoorOpen
                )
            })
            .unwrap_or(false)
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, &Tile)> {
        self.tiles.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, tile)| (x, y, tile))
        })
    }
}
