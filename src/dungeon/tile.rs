#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileType {
    Wall,
    Floor,
    Entrance,
    Exit,
    Door,
    DoorOpen,
    PlayerSpawn,
    /// Player spawn point that renders as normal floor (no GateFloor sprite).
    SpawnPoint,
    TorchWall,
    /// Empty tile - not rendered, not walkable. Used for TMX tile ID 0.
    Empty,
}

impl TileType {
    /// Whether this tile renders as floor (for edge detection).
    pub const fn is_floor(&self) -> bool {
        matches!(
            self,
            Self::Floor
                | Self::Entrance
                | Self::Exit
                | Self::DoorOpen
                | Self::PlayerSpawn
                | Self::SpawnPoint
        )
    }

    pub const fn is_walkable(&self) -> bool {
        self.is_floor()
    }

    pub const fn is_solid(&self) -> bool {
        !self.is_walkable()
    }

    pub const fn can_spawn_entity(&self) -> bool {
        matches!(self, Self::Floor | Self::DoorOpen | Self::SpawnPoint)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub tile_type: TileType,
    pub variant: u8,
    pub flip_x: bool,
    /// Original tileset ID from TMX (0 = empty, >0 = actual tile).
    /// Used for direct sprite lookup in TMX rendering.
    pub tileset_id: Option<u32>,
}

impl Tile {
    pub const fn new(tile_type: TileType) -> Self {
        Self {
            tile_type,
            variant: 0,
            flip_x: false,
            tileset_id: None,
        }
    }

    pub const fn with_variant(mut self, variant: u8) -> Self {
        self.variant = variant;
        self
    }

    pub const fn flipped(mut self) -> Self {
        self.flip_x = true;
        self
    }

    pub const fn with_tileset_id(mut self, id: u32) -> Self {
        self.tileset_id = Some(id);
        self
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self::new(TileType::Wall)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn floor_tiles_are_walkable() {
        assert!(TileType::Floor.is_walkable());
        assert!(TileType::Entrance.is_walkable());
        assert!(TileType::Exit.is_walkable());
        assert!(TileType::DoorOpen.is_walkable());
        assert!(TileType::PlayerSpawn.is_walkable());
        assert!(TileType::SpawnPoint.is_walkable());
    }

    #[test]
    fn non_floor_tiles_are_not_walkable() {
        assert!(!TileType::Wall.is_walkable());
        assert!(!TileType::Door.is_walkable());
        assert!(!TileType::TorchWall.is_walkable());
        assert!(!TileType::Empty.is_walkable());
    }

    #[test]
    fn is_floor_matches_walkable_tiles() {
        assert!(TileType::Floor.is_floor());
        assert!(TileType::Entrance.is_floor());
        assert!(TileType::Exit.is_floor());
        assert!(TileType::DoorOpen.is_floor());
        assert!(TileType::PlayerSpawn.is_floor());
        assert!(TileType::SpawnPoint.is_floor());

        assert!(!TileType::Wall.is_floor());
        assert!(!TileType::Door.is_floor());
    }

    #[test]
    fn is_solid_is_opposite_of_walkable() {
        assert!(!TileType::Floor.is_solid());
        assert!(TileType::Wall.is_solid());
        assert!(TileType::Door.is_solid());
    }

    #[test]
    fn can_spawn_entity_limited_to_spawnable_tiles() {
        assert!(TileType::Floor.can_spawn_entity());
        assert!(TileType::DoorOpen.can_spawn_entity());
        assert!(TileType::SpawnPoint.can_spawn_entity());

        assert!(!TileType::Entrance.can_spawn_entity());
        assert!(!TileType::Exit.can_spawn_entity());
        assert!(!TileType::PlayerSpawn.can_spawn_entity());
        assert!(!TileType::Wall.can_spawn_entity());
    }

    #[test]
    fn tile_new_creates_with_defaults() {
        let tile = Tile::new(TileType::Floor);
        assert_eq!(tile.tile_type, TileType::Floor);
        assert_eq!(tile.variant, 0);
        assert!(!tile.flip_x);
        assert!(tile.tileset_id.is_none());
    }

    #[test]
    fn tile_with_variant() {
        let tile = Tile::new(TileType::Floor).with_variant(3);
        assert_eq!(tile.variant, 3);
    }

    #[test]
    fn tile_flipped() {
        let tile = Tile::new(TileType::Floor).flipped();
        assert!(tile.flip_x);
    }

    #[test]
    fn tile_with_tileset_id() {
        let tile = Tile::new(TileType::Floor).with_tileset_id(42);
        assert_eq!(tile.tileset_id, Some(42));
    }

    #[test]
    fn tile_default_is_wall() {
        let tile = Tile::default();
        assert_eq!(tile.tile_type, TileType::Wall);
    }

    #[test]
    fn tile_builder_methods_chain() {
        let tile = Tile::new(TileType::Floor)
            .with_variant(2)
            .flipped()
            .with_tileset_id(10);
        assert_eq!(tile.tile_type, TileType::Floor);
        assert_eq!(tile.variant, 2);
        assert!(tile.flip_x);
        assert_eq!(tile.tileset_id, Some(10));
    }
}
