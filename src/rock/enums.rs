#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RockType {
    Copper,
    Coal,
    Tin,
}

impl RockType {
    /// Returns the sprite slice name for this rock type.
    pub fn sprite_name(&self) -> &'static str {
        match self {
            Self::Copper => "copper_rock",
            Self::Coal | Self::Tin => "coal_tin_rock",
        }
    }
}
