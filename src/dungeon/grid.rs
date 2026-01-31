use bevy::prelude::*;
use tracing::instrument;

/// Represents entity size in grid cells.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GridSize {
    pub width: u8,
    pub height: u8,
}

impl GridSize {
    pub const fn new(width: u8, height: u8) -> Self {
        Self { width, height }
    }

    /// A 1x1 single-cell entity.
    pub const fn single() -> Self {
        Self {
            width: 1,
            height: 1,
        }
    }

    /// Total number of cells occupied.
    pub const fn cells(&self) -> usize {
        self.width as usize * self.height as usize
    }

    /// Iterator over cell offsets (0,0) to (width-1, height-1).
    pub fn cell_offsets(&self) -> impl Iterator<Item = (u8, u8)> {
        let width = self.width;
        let height = self.height;
        (0..height).flat_map(move |y| (0..width).map(move |x| (x, y)))
    }
}

/// Grid position with top-left anchor convention.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GridPosition {
    pub x: usize,
    pub y: usize,
}

impl GridPosition {
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// All cells occupied by entity of given size at this position.
    pub fn occupied_cells(&self, size: GridSize) -> impl Iterator<Item = (usize, usize)> {
        let base_x = self.x;
        let base_y = self.y;
        let width = size.width;
        let height = size.height;
        (0..height).flat_map(move |dy| {
            (0..width).map(move |dx| (base_x + dx as usize, base_y + dy as usize))
        })
    }
}

#[derive(Resource)]
pub struct GridOccupancy {
    width: usize,
    height: usize,
    cells: Vec<Option<Entity>>,
    blocked: Vec<bool>,
}

impl GridOccupancy {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            cells: vec![None; size],
            blocked: vec![false; size],
        }
    }

    fn index(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height {
            Some(y * self.width + x)
        } else {
            None
        }
    }

    pub fn is_occupied(&self, x: usize, y: usize) -> bool {
        self.index(x, y)
            .map(|i| self.cells[i].is_some() || self.blocked[i])
            .unwrap_or(false)
    }

    /// Check if an entity of given size can be placed at position.
    pub fn can_place(&self, pos: GridPosition, size: GridSize) -> bool {
        pos.occupied_cells(size).all(|(x, y)| {
            self.index(x, y).is_some() && !self.is_occupied(x, y)
        })
    }

    #[instrument(level = "debug", skip(self), fields(pos = ?pos, size = ?size, entity = ?entity))]
    pub fn occupy(&mut self, pos: GridPosition, size: GridSize, entity: Entity) {
        for (x, y) in pos.occupied_cells(size) {
            if let Some(i) = self.index(x, y) {
                self.cells[i] = Some(entity);
            }
        }
    }

    pub fn mark_blocked(&mut self, pos: GridPosition, size: GridSize) {
        for (x, y) in pos.occupied_cells(size) {
            if let Some(i) = self.index(x, y) {
                self.blocked[i] = true;
            }
        }
    }

    pub fn unmark_blocked(&mut self, pos: GridPosition, size: GridSize) {
        for (x, y) in pos.occupied_cells(size) {
            if let Some(i) = self.index(x, y) {
                self.blocked[i] = false;
            }
        }
    }

    pub fn vacate(&mut self, pos: GridPosition, size: GridSize) {
        for (x, y) in pos.occupied_cells(size) {
            if let Some(i) = self.index(x, y) {
                self.cells[i] = None;
            }
        }
    }

    /// Get entity occupying a cell.
    #[instrument(level = "debug", skip(self), ret)]
    pub fn entity_at(&self, x: usize, y: usize) -> Option<Entity> {
        self.index(x, y)
            .and_then(|i| self.cells.get(i))
            .copied()
            .flatten()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_size_single() {
        let size = GridSize::single();
        assert_eq!(size.width, 1);
        assert_eq!(size.height, 1);
        assert_eq!(size.cells(), 1);
    }

    #[test]
    fn grid_size_cells() {
        let size = GridSize::new(3, 2);
        assert_eq!(size.cells(), 6);
    }

    #[test]
    fn grid_size_cell_offsets() {
        let size = GridSize::new(2, 2);
        let offsets: Vec<_> = size.cell_offsets().collect();
        assert_eq!(offsets, vec![(0, 0), (1, 0), (0, 1), (1, 1)]);
    }

    #[test]
    fn grid_position_occupied_cells() {
        let pos = GridPosition::new(5, 3);
        let size = GridSize::new(2, 2);
        let cells: Vec<_> = pos.occupied_cells(size).collect();
        assert_eq!(cells, vec![(5, 3), (6, 3), (5, 4), (6, 4)]);
    }

    #[test]
    fn grid_occupancy_basic() {
        let mut occupancy = GridOccupancy::new(10, 10);
        assert!(!occupancy.is_occupied(5, 5));

        let entity = Entity::from_raw(1);
        let pos = GridPosition::new(5, 5);
        let size = GridSize::single();

        assert!(occupancy.can_place(pos, size));
        occupancy.occupy(pos, size, entity);
        assert!(occupancy.is_occupied(5, 5));
        assert!(!occupancy.can_place(pos, size));
        assert_eq!(occupancy.entity_at(5, 5), Some(entity));

        occupancy.vacate(pos, size);
        assert!(!occupancy.is_occupied(5, 5));
        assert_eq!(occupancy.entity_at(5, 5), None);
    }

    #[test]
    fn grid_occupancy_multi_cell() {
        let mut occupancy = GridOccupancy::new(10, 10);
        let entity = Entity::from_raw(2);
        let pos = GridPosition::new(2, 2);
        let size = GridSize::new(3, 2);

        occupancy.occupy(pos, size, entity);

        // All 6 cells should be occupied
        for (x, y) in pos.occupied_cells(size) {
            assert!(occupancy.is_occupied(x, y));
            assert_eq!(occupancy.entity_at(x, y), Some(entity));
        }

        // Adjacent cells should be free
        assert!(!occupancy.is_occupied(1, 2));
        assert!(!occupancy.is_occupied(5, 2));
    }

    #[test]
    fn grid_occupancy_out_of_bounds() {
        let occupancy = GridOccupancy::new(5, 5);
        assert!(!occupancy.is_occupied(10, 10));
        assert_eq!(occupancy.entity_at(10, 10), None);

        let pos = GridPosition::new(4, 4);
        let size = GridSize::new(2, 2); // Would extend past grid
        assert!(!occupancy.can_place(pos, size));
    }
}
