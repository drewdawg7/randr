use super::entity::DungeonEntity;
use super::grid::{GridPosition, GridSize};
use super::tile::Tile;

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
            .map(|t| t.tile_type.is_floor())
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

    /// Returns the entity only if (x, y) is its anchor (top-left) position.
    /// Use this for rendering to avoid drawing multi-cell entities multiple times.
    pub fn entity_anchored_at(&self, x: usize, y: usize) -> Option<&DungeonEntity> {
        self.entities
            .iter()
            .find(|(pos, _)| pos.x == x && pos.y == y)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dungeon::{Tile, TileType};

    fn create_floor_layout(width: usize, height: usize) -> DungeonLayout {
        let mut layout = DungeonLayout::new(width, height);
        for y in 0..height {
            for x in 0..width {
                layout.set_tile(x, y, Tile::new(TileType::Floor));
            }
        }
        layout
    }

    #[test]
    fn new_creates_layout_with_dimensions() {
        let layout = DungeonLayout::new(10, 8);
        assert_eq!(layout.width(), 10);
        assert_eq!(layout.height(), 8);
    }

    #[test]
    fn new_layout_has_default_entrance() {
        let layout = DungeonLayout::new(5, 5);
        assert_eq!(layout.entrance, (0, 0));
    }

    #[test]
    fn new_layout_has_no_exit() {
        let layout = DungeonLayout::new(5, 5);
        assert!(layout.exit.is_none());
    }

    #[test]
    fn new_layout_has_no_entities() {
        let layout = DungeonLayout::new(5, 5);
        assert!(layout.entities().is_empty());
    }

    #[test]
    fn tile_at_returns_tile() {
        let layout = DungeonLayout::new(5, 5);
        let tile = layout.tile_at(2, 2);
        assert!(tile.is_some());
    }

    #[test]
    fn tile_at_out_of_bounds_returns_none() {
        let layout = DungeonLayout::new(5, 5);
        assert!(layout.tile_at(10, 10).is_none());
        assert!(layout.tile_at(5, 0).is_none());
        assert!(layout.tile_at(0, 5).is_none());
    }

    #[test]
    fn set_tile_updates_tile() {
        let mut layout = DungeonLayout::new(5, 5);
        layout.set_tile(2, 2, Tile::new(TileType::Floor));
        let tile = layout.tile_at(2, 2).expect("tile should exist");
        assert_eq!(tile.tile_type, TileType::Floor);
    }

    #[test]
    fn is_walkable_for_floor() {
        let mut layout = DungeonLayout::new(5, 5);
        layout.set_tile(2, 2, Tile::new(TileType::Floor));
        assert!(layout.is_walkable(2, 2));
    }

    #[test]
    fn is_walkable_for_wall() {
        let layout = DungeonLayout::new(5, 5);
        assert!(!layout.is_walkable(0, 0));
    }

    #[test]
    fn is_walkable_out_of_bounds() {
        let layout = DungeonLayout::new(5, 5);
        assert!(!layout.is_walkable(10, 10));
    }

    #[test]
    fn is_floor_for_floor_tile() {
        let mut layout = DungeonLayout::new(5, 5);
        layout.set_tile(1, 1, Tile::new(TileType::Floor));
        assert!(layout.is_floor(1, 1));
    }

    #[test]
    fn is_floor_for_wall_tile() {
        let layout = DungeonLayout::new(5, 5);
        assert!(!layout.is_floor(0, 0));
    }

    #[test]
    fn iter_visits_all_tiles() {
        let layout = DungeonLayout::new(3, 3);
        let tiles: Vec<_> = layout.iter().collect();
        assert_eq!(tiles.len(), 9);
    }

    #[test]
    fn iter_includes_coordinates() {
        let layout = DungeonLayout::new(2, 2);
        let coords: Vec<_> = layout.iter().map(|(x, y, _)| (x, y)).collect();
        assert!(coords.contains(&(0, 0)));
        assert!(coords.contains(&(1, 0)));
        assert!(coords.contains(&(0, 1)));
        assert!(coords.contains(&(1, 1)));
    }

    #[test]
    fn spawn_points_returns_spawnable_tiles() {
        let mut layout = DungeonLayout::new(5, 5);
        layout.set_tile(1, 1, Tile::new(TileType::Floor));
        layout.set_tile(2, 2, Tile::new(TileType::Floor));
        layout.set_tile(3, 3, Tile::new(TileType::Entrance));

        let spawn_points = layout.spawn_points();
        assert!(spawn_points.contains(&(1, 1)));
        assert!(spawn_points.contains(&(2, 2)));
        assert!(!spawn_points.contains(&(3, 3)));
    }

    #[test]
    fn add_entity_stores_entity() {
        let mut layout = create_floor_layout(5, 5);
        let pos = GridPosition::new(2, 2);
        layout.add_entity(
            pos,
            DungeonEntity::Chest {
                variant: 0,
                size: GridSize::single(),
            },
        );
        assert_eq!(layout.entities().len(), 1);
    }

    #[test]
    fn entity_at_finds_entity() {
        let mut layout = create_floor_layout(5, 5);
        let pos = GridPosition::new(2, 2);
        layout.add_entity(
            pos,
            DungeonEntity::Chest {
                variant: 0,
                size: GridSize::single(),
            },
        );
        assert!(layout.entity_at(2, 2).is_some());
    }

    #[test]
    fn entity_at_returns_none_for_empty_cell() {
        let layout = create_floor_layout(5, 5);
        assert!(layout.entity_at(2, 2).is_none());
    }

    #[test]
    fn entity_at_finds_multi_cell_entity() {
        let mut layout = create_floor_layout(10, 10);
        let pos = GridPosition::new(2, 2);
        let size = GridSize::new(2, 2);
        layout.add_entity(
            pos,
            DungeonEntity::Chest { variant: 0, size },
        );
        assert!(layout.entity_at(2, 2).is_some());
        assert!(layout.entity_at(3, 2).is_some());
        assert!(layout.entity_at(2, 3).is_some());
        assert!(layout.entity_at(3, 3).is_some());
    }

    #[test]
    fn entity_anchored_at_only_matches_anchor() {
        let mut layout = create_floor_layout(10, 10);
        let pos = GridPosition::new(2, 2);
        let size = GridSize::new(2, 2);
        layout.add_entity(
            pos,
            DungeonEntity::Chest { variant: 0, size },
        );
        assert!(layout.entity_anchored_at(2, 2).is_some());
        assert!(layout.entity_anchored_at(3, 2).is_none());
        assert!(layout.entity_anchored_at(2, 3).is_none());
    }

    #[test]
    fn spawn_areas_finds_valid_positions() {
        let layout = create_floor_layout(5, 5);
        let areas = layout.spawn_areas(GridSize::single());
        assert_eq!(areas.len(), 25);
    }

    #[test]
    fn spawn_areas_excludes_positions_with_entities() {
        let mut layout = create_floor_layout(5, 5);
        layout.add_entity(
            GridPosition::new(2, 2),
            DungeonEntity::Chest {
                variant: 0,
                size: GridSize::single(),
            },
        );
        let areas = layout.spawn_areas(GridSize::single());
        assert_eq!(areas.len(), 24);
        assert!(!areas.contains(&GridPosition::new(2, 2)));
    }

    #[test]
    fn spawn_areas_excludes_non_floor_tiles() {
        let mut layout = DungeonLayout::new(5, 5);
        layout.set_tile(2, 2, Tile::new(TileType::Floor));
        let areas = layout.spawn_areas(GridSize::single());
        assert_eq!(areas.len(), 1);
        assert!(areas.contains(&GridPosition::new(2, 2)));
    }

    #[test]
    fn spawn_areas_for_multi_cell_entity() {
        let layout = create_floor_layout(5, 5);
        let size = GridSize::new(2, 2);
        let areas = layout.spawn_areas(size);
        assert_eq!(areas.len(), 16);
    }

    #[test]
    fn areas_overlap_detects_overlap() {
        let p1 = GridPosition::new(0, 0);
        let s1 = GridSize::new(2, 2);
        let p2 = GridPosition::new(1, 1);
        let s2 = GridSize::new(2, 2);
        assert!(DungeonLayout::areas_overlap(p1, s1, p2, s2));
    }

    #[test]
    fn areas_overlap_detects_no_overlap() {
        let p1 = GridPosition::new(0, 0);
        let s1 = GridSize::new(2, 2);
        let p2 = GridPosition::new(5, 5);
        let s2 = GridSize::new(2, 2);
        assert!(!DungeonLayout::areas_overlap(p1, s1, p2, s2));
    }

    #[test]
    fn areas_overlap_adjacent_not_overlapping() {
        let p1 = GridPosition::new(0, 0);
        let s1 = GridSize::new(2, 2);
        let p2 = GridPosition::new(2, 0);
        let s2 = GridSize::new(2, 2);
        assert!(!DungeonLayout::areas_overlap(p1, s1, p2, s2));
    }
}
