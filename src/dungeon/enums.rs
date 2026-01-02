#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoomType {
    Monster,
    Boss,
    Rest,
    Trap,
    Chest,
    Treasure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    /// Returns the offset (dx, dy) for this direction
    pub fn offset(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }

    /// Returns the display name for this direction
    pub fn name(&self) -> &'static str {
        match self {
            Direction::North => "North",
            Direction::East => "East",
            Direction::South => "South",
            Direction::West => "West",
        }
    }

    /// Returns all four directions
    pub fn all() -> [Direction; 4] {
        [Direction::North, Direction::East, Direction::South, Direction::West]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DungeonError {
    NoRoomAtPosition,
    InvalidDirection,
    MobSpawnError,
}
