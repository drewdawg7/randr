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
