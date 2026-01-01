# Adding New Locations

## Quick Checklist

1. [ ] Add `LocationId` variant to `enums.rs`
2. [ ] Add subtype to relevant category in `enums.rs`
3. [ ] Update `LocationId::location_type()` match
4. [ ] Add `LocationData` variant to `spec/definition.rs`
5. [ ] Create location submodule (`src/location/<name>/`)
6. [ ] Add spec to `spec/specs.rs`
7. [ ] Export from `location/mod.rs`
8. [ ] Add field to Town in `town/definition.rs`
9. [ ] Update Town helper methods

## Step-by-Step Guide

### 1. Add LocationId Variant (enums.rs)

```rust
pub enum LocationId {
    VillageStore,
    VillageBlacksmith,
    VillageField,
    VillageMine,
    VillageInn,  // NEW
}
```

### 2. Add Subtype (enums.rs)

If new category:
```rust
pub enum LocationType {
    Commerce(CommerceSubtype),
    Crafting(CraftingSubtype),
    Combat(CombatSubtype),
    Resource(ResourceSubtype),
    Service(ServiceSubtype),  // NEW category
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ServiceSubtype {
    Inn,
}
```

### 3. Update location_type() Match (enums.rs)

```rust
impl LocationId {
    pub fn location_type(&self) -> LocationType {
        match self {
            // ...existing...
            LocationId::VillageInn => LocationType::Service(ServiceSubtype::Inn),
        }
    }
}
```

### 4. Add LocationData Variant (spec/definition.rs)

```rust
pub enum LocationData {
    Store(StoreData),
    Blacksmith(BlacksmithData),
    Field(FieldData),
    Mine(MineData),
    Inn(InnData),  // NEW
}

#[derive(Clone)]
pub struct InnData {
    pub rest_cost: i32,
    pub heal_percent: f32,
}
```

### 5. Create Location Submodule

Create `src/location/inn/`:

```
inn/
├── mod.rs
├── definition.rs
├── traits.rs
└── enums.rs (if needed)
```

**mod.rs:**
```rust
mod definition;
mod traits;

pub use definition::Inn;
```

**definition.rs:**
```rust
use crate::location::{InnData, LocationId, LocationSpec};

pub struct Inn {
    pub(crate) location_id: LocationId,
    pub name: String,
    pub(crate) description: String,
    pub rest_cost: i32,
    pub heal_percent: f32,
}

impl Inn {
    pub fn from_spec(spec: &LocationSpec, data: &InnData) -> Self {
        Inn {
            location_id: spec.location_id,
            name: spec.name.to_string(),
            description: spec.description.to_string(),
            rest_cost: data.rest_cost,
            heal_percent: data.heal_percent,
        }
    }

    pub fn location_id(&self) -> LocationId {
        self.location_id
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}
```

**traits.rs:**
```rust
use std::time::Duration;
use crate::{
    entities::Player,
    location::{ActivityId, Location, LocationEntryError, LocationId},
};
use super::definition::Inn;

impl Default for Inn {
    fn default() -> Self {
        Self {
            location_id: LocationId::VillageInn,
            name: "Village Inn".to_string(),
            description: "A cozy inn to rest and restore health".to_string(),
            rest_cost: 10,
            heal_percent: 0.5,
        }
    }
}

impl Location for Inn {
    fn id(&self) -> LocationId { self.location_id() }
    fn name(&self) -> &str { &self.name }
    fn description(&self) -> &str { Inn::description(self) }
    fn tick(&mut self, _elapsed: Duration) {}
    fn refresh(&mut self) {}
    fn time_until_refresh(&self) -> Option<Duration> { None }
    fn can_enter(&self, _player: &Player) -> Result<(), LocationEntryError> { Ok(()) }
    fn on_enter(&mut self, _player: &mut Player) {}
    fn on_exit(&mut self, _player: &mut Player) {}
    fn available_activities(&self) -> &[ActivityId] { &[ActivityId::Rest] }
}
```

### 6. Add Spec (spec/specs.rs)

```rust
pub static VILLAGE_INN: Lazy<LocationSpec> = Lazy::new(|| LocationSpec {
    location_id: LocationId::VillageInn,
    name: "Village Inn",
    description: "A cozy inn to rest and restore health",
    refresh_interval: None,
    min_level: None,
    activities: vec![ActivitySpec::new(
        ActivityId::Rest,
        "Rest",
        "Restore health for gold",
    )],
    data: LocationData::Inn(InnData {
        rest_cost: 10,
        heal_percent: 0.5,
    }),
});
```

### 7. Export from location/mod.rs

```rust
pub mod inn;
// ...
pub use inn::Inn;
```

### 8. Add to Town (town/definition.rs)

```rust
pub struct Town {
    pub name: String,
    pub store: Store,
    pub blacksmith: Blacksmith,
    pub field: Field,
    pub mine: Mine,
    pub inn: Inn,  // NEW
}
```

Update `new()`, `location()`, `location_mut()`, `tick_all()`.

### 9. Add Activity (if new)

In `activity.rs`:
```rust
pub enum ActivityId {
    // ...existing...
    Rest,  // NEW
}
```

## Testing

Always run after changes:
```bash
cargo check
cargo test
```

## Common Patterns

### Location-specific methods
Add to `definition.rs`, not traits.rs:
```rust
impl Inn {
    pub fn rest(&mut self, player: &mut Player) -> Result<(), InnError> {
        // ...
    }
}
```

### Accessing from GameState
```rust
// Direct field access
game_state().town.inn.rest(player)?;

// Via trait
let loc = game_state().town.location_mut(LocationId::VillageInn);
loc.on_enter(player);
```
