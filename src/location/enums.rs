/// Identifies specific location instances (like ItemId, MobId)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LocationId {
    // Commerce locations
    VillageStore,
    // Crafting locations
    VillageBlacksmith,
    // Combat locations
    VillageField,
    // Resource locations
    VillageMine,
}

/// High-level location categories with nested subtypes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LocationType {
    Commerce(CommerceSubtype),
    Crafting(CraftingSubtype),
    Combat(CombatSubtype),
    Resource(ResourceSubtype),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommerceSubtype {
    Store,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CraftingSubtype {
    Blacksmith,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CombatSubtype {
    Field,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceSubtype {
    Mine,
}

impl LocationId {
    /// Get the category type for this location
    pub fn location_type(&self) -> LocationType {
        match self {
            LocationId::VillageStore => LocationType::Commerce(CommerceSubtype::Store),
            LocationId::VillageBlacksmith => LocationType::Crafting(CraftingSubtype::Blacksmith),
            LocationId::VillageField => LocationType::Combat(CombatSubtype::Field),
            LocationId::VillageMine => LocationType::Resource(ResourceSubtype::Mine),
        }
    }
}
