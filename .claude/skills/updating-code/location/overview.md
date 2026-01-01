# Location System Overview

## Module Structure

```
src/location/
├── mod.rs              # Main exports
├── enums.rs            # LocationId, LocationType, subtypes
├── traits.rs           # Location trait, LocationEntryError
├── activity.rs         # ActivityId, ActivitySpec
├── spec/
│   ├── mod.rs
│   ├── definition.rs   # LocationSpec, LocationData, StoreData, etc.
│   ├── traits.rs       # SpawnFromSpec, RegistryDefaults
│   └── specs.rs        # VILLAGE_STORE, VILLAGE_BLACKSMITH, etc.
├── store/
│   ├── mod.rs
│   ├── definition.rs   # Store struct
│   ├── traits.rs       # Location + Default impls
│   └── store_item.rs   # StoreItem struct
├── blacksmith/
│   ├── mod.rs
│   ├── definition.rs   # Blacksmith struct
│   ├── traits.rs       # Location + Default impls
│   ├── tests.rs
│   └── enums.rs        # BlacksmithError
├── field/
│   ├── mod.rs
│   ├── definition.rs   # Field struct
│   ├── traits.rs       # Location + Default impls
│   └── enums.rs        # FieldError, FieldId
└── mine/
    ├── mod.rs
    ├── definition.rs   # Mine struct
    ├── traits.rs       # Location + Default impls
    ├── tests.rs
    └── rock/           # Rock submodule (RockId, RockRegistry, etc.)
```

## Key Types

### LocationId (enums.rs)
Identifies specific location instances (like ItemId, MobId):
- `VillageStore`
- `VillageBlacksmith`
- `VillageField`
- `VillageMine`

### LocationType (enums.rs)
Categories with nested subtypes:
- `Commerce(CommerceSubtype)` - Store
- `Crafting(CraftingSubtype)` - Blacksmith
- `Combat(CombatSubtype)` - Field
- `Resource(ResourceSubtype)` - Mine

### Location Trait (traits.rs)
Core trait all locations implement:
- **Identity**: `id()`, `name()`, `description()`, `location_type()`
- **Timer/Refresh**: `tick()`, `refresh()`, `time_until_refresh()`
- **Entry/Exit**: `can_enter()`, `on_enter()`, `on_exit()`
- **Activities**: `available_activities()`, `is_activity_available()`

### ActivityId (activity.rs)
Activities available at locations:
- `Buy`, `Sell` (Store)
- `Upgrade`, `UpgradeQuality`, `Smelt`, `Forge` (Blacksmith)
- `Fight` (Field)
- `MineRock` (Mine)

### LocationSpec (spec/definition.rs)
Unified spec for all locations:
```rust
pub struct LocationSpec {
    pub location_id: LocationId,
    pub name: &'static str,
    pub description: &'static str,
    pub refresh_interval: Option<Duration>,
    pub min_level: Option<i32>,
    pub activities: Vec<ActivitySpec>,
    pub data: LocationData,  // Location-specific config
}
```

### LocationData (spec/definition.rs)
Location-specific configuration:
```rust
pub enum LocationData {
    Store(StoreData),
    Blacksmith(BlacksmithData),
    Field(FieldData),
    Mine(MineData),
}
```

## Town Integration (town/definition.rs)

Town holds all locations as named fields:
```rust
pub struct Town {
    pub name: String,
    pub store: Store,
    pub blacksmith: Blacksmith,
    pub field: Field,
    pub mine: Mine,
}
```

Helper methods:
- `location(&self, id: LocationId) -> &dyn Location`
- `location_mut(&mut self, id: LocationId) -> &mut dyn Location`
- `tick_all(&mut self, elapsed: Duration)`

## Spawning Locations

Locations are created via `Default::default()`:
```rust
let store = Store::default();
let blacksmith = Blacksmith::new("Village Blacksmith".to_string(), 10, 50);
let field = Field::default();
let mine = Mine::default();
```

Or from specs:
```rust
let store = Store::from_spec(&VILLAGE_STORE, &store_data);
```

## Files to Modify for Location Changes

| Change | Files |
|--------|-------|
| Add new location type | `enums.rs`, `traits.rs`, `spec/definition.rs`, `spec/specs.rs`, new submodule |
| Add new activity | `activity.rs`, relevant location's traits.rs |
| Modify location behavior | `<location>/definition.rs`, `<location>/traits.rs` |
| Add location to town | `town/definition.rs` |
