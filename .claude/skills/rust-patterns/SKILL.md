# Rust Patterns Skill

## Project-Specific Patterns

### Registry Pattern
Used for: Items, Mobs, Rocks

```rust
// Define a spec (static data)
pub struct ItemSpec {
    pub name: String,
    pub item_type: ItemType,
}

// Register defaults
impl RegistryDefaults<ItemId> for ItemSpec {
    fn defaults() -> Vec<(ItemId, Self)> { ... }
}

// Spawn instances from specs
impl SpawnFromSpec<ItemId> for Item {
    fn spawn(id: &ItemId, spec: &ItemSpec) -> Self { ... }
}
```

### Trait Composition
Core behaviors via traits:

```rust
// Combat
impl Combatant for Player { ... }
impl HasStats for Player { ... }

// Progression
impl HasProgression for Player { ... }
impl GivesXP for Mob { ... }

// Inventory
impl HasInventory for Player { ... }
```

### Global State Access
```rust
use crate::system::game_state;

// Access state (requires unsafe)
let state = game_state();
state.player.take_damage(10);
```

## Common Patterns

### Builder Pattern
```rust
Item::new(ItemId::Sword)
    .with_damage(10)
    .with_durability(100)
```

### Error Handling
```rust
// Return Result for fallible operations
fn do_thing() -> Result<T, Error> { ... }

// Use ? for propagation
let result = do_thing()?;
```

### Option Chaining
```rust
player.inventory
    .get_item(id)
    .map(|item| item.damage)
    .unwrap_or(0)
```

## Re-export Pattern

For backward-compatible module reorganization, prefer re-exports over updating every import:

```rust
// old_module.rs - keep re-exports for compatibility
pub use new_module::Thing;

// new_module.rs - actual implementation
pub struct Thing { ... }
```

This is simpler than transforming imports across many files.
