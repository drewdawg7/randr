/// Enum of all possible activities across all locations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActivityId {
    // Commerce activities
    Buy,
    Sell,

    // Crafting activities
    Upgrade,
    UpgradeQuality,
    Smelt,
    Forge,

    // Combat activities
    Fight,

    // Resource activities
    MineRock,
}

/// Specification for an activity that a location offers
#[derive(Clone)]
pub struct ActivitySpec {
    pub id: ActivityId,
    pub name: &'static str,
    pub description: &'static str,
}

impl ActivitySpec {
    pub const fn new(id: ActivityId, name: &'static str, description: &'static str) -> Self {
        Self {
            id,
            name,
            description,
        }
    }
}
