use super::entity::DungeonEntity;
use super::grid::{GridPosition, GridSize};
use super::tile::{Tile, TileType};

#[derive(Debug, Clone)]
pub struct DungeonLayout {
    width: usize,
    height: usize,
    tiles: Vec<Vec<Tile>>,
    pub entrance: (usize, usize),
    pub exit: Option<(usize, usize)>,
    entities: Vec<(GridPosition, DungeonEntity)>,
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
            entities: Vec::new(),
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
                    TileType::Floor
                        | TileType::Entrance
                        | TileType::Exit
                        | TileType::DoorOpen
                        | TileType::PlayerSpawn
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

    pub fn spawn_points(&self) -> Vec<(usize, usize)> {
        self.iter()
            .filter(|(_, _, tile)| tile.tile_type.can_spawn_entity())
            .map(|(x, y, _)| (x, y))
            .collect()
    }

    pub fn add_entity(&mut self, pos: GridPosition, entity: DungeonEntity) {
        self.entities.push((pos, entity));
    }

    pub fn entities(&self) -> &[(GridPosition, DungeonEntity)] {
        &self.entities
    }

    /// Returns the entity occupying the given cell, considering entity sizes.
    pub fn entity_at(&self, x: usize, y: usize) -> Option<&DungeonEntity> {
        self.entities
            .iter()
            .find(|(pos, entity)| {
                pos.occupied_cells(entity.size())
                    .any(|(cx, cy)| cx == x && cy == y)
            })
            .map(|(_, e)| e)
    }

    /// Find all valid spawn positions for an entity of given size.
    pub fn spawn_areas(&self, size: GridSize) -> Vec<GridPosition> {
        let mut valid = Vec::new();

        for y in 0..self.height.saturating_sub(size.height as usize - 1) {
            for x in 0..self.width.saturating_sub(size.width as usize - 1) {
                let pos = GridPosition::new(x, y);

                // Check all cells are spawnable floor tiles
                let all_floor = pos
                    .occupied_cells(size)
                    .all(|(cx, cy)| self.is_floor(cx, cy));

                // Check no existing entity occupies these cells
                let no_overlap = !self
                    .entities
                    .iter()
                    .any(|(epos, entity)| Self::areas_overlap(pos, size, *epos, entity.size()));

                if all_floor && no_overlap {
                    valid.push(pos);
                }
            }
        }
        valid
    }

    fn areas_overlap(p1: GridPosition, s1: GridSize, p2: GridPosition, s2: GridSize) -> bool {
        let r1 = p1.x + s1.width as usize;
        let b1 = p1.y + s1.height as usize;
        let r2 = p2.x + s2.width as usize;
        let b2 = p2.y + s2.height as usize;

        p1.x < r2 && r1 > p2.x && p1.y < b2 && b1 > p2.y
    }
}
