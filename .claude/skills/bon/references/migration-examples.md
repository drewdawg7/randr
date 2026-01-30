# Migration Examples

Before/after examples from this codebase.

## Column Widget

### Before (Manual Builder)

```rust
// src/ui/widgets/column.rs
#[derive(Component)]
pub struct Column {
    pub gap: f32,
    pub justify: JustifyContent,
    pub align: AlignItems,
    pub padding: UiRect,
    pub width: Option<Val>,
    pub height: Option<Val>,
}

impl Column {
    pub fn new() -> Self {
        Self {
            gap: 0.0,
            justify: JustifyContent::FlexStart,
            align: AlignItems::Stretch,
            padding: UiRect::default(),
            width: None,
            height: None,
        }
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn justify(mut self, justify: JustifyContent) -> Self {
        self.justify = justify;
        self
    }
    // ... more methods
}

// Usage
Column::new().gap(5.0).align(AlignItems::Center)
```

### After (bon Builder)

```rust
use bon::Builder;

#[derive(Component, Builder)]
pub struct Column {
    #[builder(default = 0.0)]
    pub gap: f32,

    #[builder(default = JustifyContent::FlexStart)]
    pub justify: JustifyContent,

    #[builder(default = AlignItems::Stretch)]
    pub align: AlignItems,

    #[builder(default)]
    pub padding: UiRect,

    #[builder(default)]
    pub width: Option<Val>,

    #[builder(default)]
    pub height: Option<Val>,
}

// Usage
Column::builder().gap(5.0).align(AlignItems::Center).build()
```

## SpawnTable (Vec Accumulator)

### Before (Manual Accumulator)

```rust
// src/dungeon/spawn.rs
pub struct SpawnTable {
    entries: Vec<SpawnEntry>,
    mob_count: RangeInclusive<u32>,
    guaranteed_mobs: Vec<(MobId, u32)>,
    // ...
}

impl SpawnTable {
    pub fn new() -> Self { ... }

    pub fn mob(mut self, mob_id: MobId, weight: u32) -> Self {
        let size = mob_id.spec().grid_size;
        self.entries.push(SpawnEntry { ... });
        self
    }

    pub fn guaranteed_mob(mut self, mob_id: MobId, count: u32) -> Self {
        self.guaranteed_mobs.push((mob_id, count));
        self
    }
}
```

### After (bon with `#[builder(field)]`)

```rust
use bon::Builder;

#[derive(Builder)]
pub struct SpawnTable {
    #[builder(field)]
    entries: Vec<SpawnEntry>,

    #[builder(default = 0..=0)]
    mob_count: RangeInclusive<u32>,

    #[builder(field)]
    guaranteed_mobs: Vec<(MobId, u32)>,
    // ...
}

use spawn_table_builder::State;

impl<S: State> SpawnTableBuilder<S> {
    pub fn mob(mut self, mob_id: MobId, weight: u32) -> Self {
        let size = mob_id.spec().grid_size;
        self.entries.push(SpawnEntry {
            entity_type: SpawnEntityType::Mob(mob_id),
            weight,
            size,
        });
        self
    }

    pub fn guaranteed_mob(mut self, mob_id: MobId, count: u32) -> Self {
        self.guaranteed_mobs.push((mob_id, count));
        self
    }
}

// Usage unchanged
SpawnTable::builder()
    .mob(MobId::Goblin, 1)
    .mob_count(1..=1)
    .build()
```
