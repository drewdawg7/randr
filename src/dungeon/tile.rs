#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileType {
    Wall,
    Floor,
    Entrance,
    Exit,
    Door,
    DoorOpen,
}

impl TileType {
    pub const fn is_walkable(&self) -> bool {
        matches!(
            self,
            Self::Floor | Self::Entrance | Self::Exit | Self::DoorOpen
        )
    }

    pub const fn is_solid(&self) -> bool {
        !self.is_walkable()
    }

    pub const fn can_spawn_entity(&self) -> bool {
        matches!(self, Self::Floor | Self::DoorOpen)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub tile_type: TileType,
    pub variant: u8,
    pub flip_x: bool,
}

impl Tile {
    pub const fn new(tile_type: TileType) -> Self {
        Self {
            tile_type,
            variant: 0,
            flip_x: false,
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
}

impl Default for Tile {
    fn default() -> Self {
        Self::new(TileType::Wall)
    }
}
