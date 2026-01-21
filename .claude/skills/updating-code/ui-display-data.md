# UI Display Data Abstractions

Pattern for decoupling UI rendering from game entity registries.

## Problem

UI code directly accessing game registries (e.g., `MobId::ALL`, `ItemId::spec()`) creates tight coupling between UI and game data structures.

## Solution

Create display data abstractions that:
1. Pre-compute display data before rendering
2. Provide a unified interface for different data sources
3. Keep render functions pure (no registry access)

## Patterns in Codebase

### 1. Source Enum Pattern (`InfoPanelSource`)

**File:** `src/screens/town/tabs/store/render/mod.rs`

Enum indicates *where* to fetch data, helper functions extract it:

```rust
pub enum InfoPanelSource {
    Store { selected_index: usize },
    Inventory { selected_index: usize },
}

// Helper in panels.rs extracts actual data
fn get_item_from_source<'a>(
    source: &InfoPanelSource,
    store: &'a Store,
    inventory: &'a Inventory,
) -> Option<&'a Item> {
    match source {
        InfoPanelSource::Store { selected_index } => {
            store.inventory.get(*selected_index)?.display_item()
        }
        InfoPanelSource::Inventory { selected_index } => {
            inventory.items.get(*selected_index).map(|i| &i.item)
        }
    }
}
```

### 2. Wrapper Enum Pattern (`ItemInfo`)

**File:** `src/screens/inventory_modal/state.rs`

Enum wraps different item sources with accessor methods:

```rust
pub enum ItemInfo {
    Equipped(EquipmentSlot, Item),
    Backpack(uuid::Uuid, InventoryItem),
}

impl ItemInfo {
    pub fn item(&self) -> &Item { ... }
    pub fn quantity(&self) -> u32 { ... }
    pub fn is_equipped(&self) -> bool { ... }
}
```

### 3. Display Struct Pattern (`ItemGridEntry`)

**File:** `src/ui/widgets/item_grid.rs`

Pure display struct with only rendering-relevant fields:

```rust
pub struct ItemGridEntry {
    pub sprite_name: String,
}
```

### 4. Resource Pattern (`CompendiumMonsters`)

**File:** `src/screens/monster_compendium.rs`

Resource holds pre-computed list of display entries:

```rust
pub struct MonsterEntry {
    pub name: String,
    pub mob_id: MobId,
}

#[derive(Resource)]
pub struct CompendiumMonsters(pub Vec<MonsterEntry>);

impl CompendiumMonsters {
    pub fn from_registry() -> Self {
        Self(
            MobId::ALL
                .iter()
                .map(|mob_id| MonsterEntry {
                    name: mob_id.spec().name.clone(),
                    mob_id: *mob_id,
                })
                .collect(),
        )
    }
}
```

Insert resource when UI opens, remove when it closes:

```rust
// Opening
commands.insert_resource(CompendiumMonsters::from_registry());

// Closing
commands.remove_resource::<CompendiumMonsters>();
```

## When to Use Each Pattern

| Pattern | Use When |
|---------|----------|
| Source Enum | UI displays data from multiple sources with same structure |
| Wrapper Enum | Different data types need unified interface |
| Display Struct | Widget needs minimal data, maximum decoupling |
| Resource | List needs iteration/indexing, lifecycle tied to UI |

## Benefits

- Render functions don't call `.spec()` or access `*Id::ALL`
- Registry changes don't break UI code
- Easier to test UI in isolation
- Clear data flow: registry -> display data -> UI
