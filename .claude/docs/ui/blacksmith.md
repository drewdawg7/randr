# Blacksmith Module

## Overview
The blacksmith module (`src/game/blacksmith.rs`) handles item upgrades, quality upgrades, smelting, and forging operations.

## Events

| Event | Purpose |
|-------|---------|
| `UpgradeItemEvent` | Upgrade item stats (costs gold) |
| `UpgradeQualityEvent` | Upgrade item quality (costs QualityUpgradeStone) |
| `SmeltRecipeEvent` | Smelt ore into bars |
| `ForgeRecipeEvent` | Forge equipment from materials |
| `BlacksmithResult` | Result enum for all operations (read by UI) |

## Crafting Helper Pattern

`handle_smelt_recipe` and `handle_forge_recipe` share logic via the `process_crafting_recipe` helper.

### CraftingOperation Enum

```rust
enum CraftingOperation {
    Smelt,
    Forge,
}
```

Methods on `CraftingOperation` generate the appropriate `BlacksmithResult` variants:
- `success_result(item_name)` → `SmeltSuccess` or `ForgeSuccess`
- `fail_ingredients_result(recipe_name)` → `SmeltFailedInsufficientIngredients` or `ForgeFailedInsufficientIngredients`
- `fail_full_result(item_name)` → `SmeltFailedInventoryFull` or `ForgeFailedInventoryFull`
- `verb()` → "smelt" or "forge" (for log messages)
- `past_verb()` → "Smelted" or "Forged" (for log messages)

### Helper Function

```rust
fn process_crafting_recipe(
    recipe_id: RecipeId,
    operation: CraftingOperation,
    result_events: &mut EventWriter<BlacksmithResult>,
    player: &mut Player,
) -> bool  // Returns true if successful, caller should write_back
```

### Handler Pattern

```rust
fn handle_smelt_recipe(/* system params */) {
    for event in smelt_events.read() {
        let mut player = Player::from_resources(&name, &gold, &progression, &inventory, &stats);
        if process_crafting_recipe(event.recipe_id, CraftingOperation::Smelt, &mut result_events, &mut player) {
            player.write_back(&mut gold, &mut progression, &mut inventory, &mut stats);
        }
    }
}
```

## Adding New Crafting Operations

1. Add new variant to `CraftingOperation` enum
2. Add corresponding `BlacksmithResult` variants
3. Extend each method on `CraftingOperation` to handle the new variant
4. Create the event type and handler system following the pattern above

## Upgrade Operation Pattern

Item upgrades (stat and quality) use the `UpgradeOperation` enum in `src/location/blacksmith/enums.rs`. This pattern consolidates the shared logic for finding items in inventory/equipment and applying upgrades.

### UpgradeOperation Enum

```rust
pub enum UpgradeOperation {
    /// Upgrade item stats (costs gold)
    Stat { max_upgrades: i32, base_upgrade_cost: i32 },
    /// Upgrade item quality (costs QualityUpgradeStone)
    Quality,
}
```

### Execute Method

The `execute` method handles validation, resource consumption, and the upgrade itself:

```rust
impl UpgradeOperation {
    pub fn execute(
        self,
        player: &mut Player,
        item: &mut Item,
    ) -> Result<UpgradeOperationResult, BlacksmithError>
}
```

### Result Types

```rust
pub enum UpgradeOperationResult {
    StatUpgrade(BlacksmithUpgradeResult),
    QualityUpgrade(ItemQuality),
}
```

### Usage Pattern

For direct item upgrades (e.g., in tests):

```rust
let operation = UpgradeOperation::Stat {
    max_upgrades: blacksmith.max_upgrades,
    base_upgrade_cost: blacksmith.base_upgrade_cost,
};
match operation.execute(&mut player, &mut item)? {
    UpgradeOperationResult::StatUpgrade(result) => Ok(result),
    UpgradeOperationResult::QualityUpgrade(_) => unreachable!(),
}
```

For UUID-based lookups (finding items in inventory/equipment):

```rust
// In definition.rs - process_player_upgrade handles the find-remove-modify-reinsert pattern
blacksmith.upgrade_player_item(player, item_uuid)
blacksmith.upgrade_player_item_quality(player, item_uuid)
```

### Adding New Upgrade Operations

1. Add new variant to `UpgradeOperation` enum
2. Add corresponding variant to `UpgradeOperationResult`
3. Extend the `execute` method to handle the new variant
4. Add a public method to `Blacksmith` that extracts the specific result type

## Recipe System

### Location
`src/item/recipe/specs.rs` - Recipe definitions using `entity_macros::define_data!`

### Cached Recipe Lists

Recipe filtering methods use `LazyLock` for lazy static initialization. This avoids repeated filtering each frame:

```rust
static FORGING_RECIPES: LazyLock<Vec<RecipeId>> = LazyLock::new(|| {
    RecipeId::ALL.iter()
        .filter(|id| id.spec().recipe_type == RecipeType::Forging)
        .copied()
        .collect()
});

impl RecipeId {
    pub fn all_forging_recipes() -> &'static [RecipeId] {
        &FORGING_RECIPES
    }
}
```

Available cached methods:
- `RecipeId::all_forging_recipes()` → `&'static [RecipeId]`
- `RecipeId::all_alchemy_recipes()` → `&'static [RecipeId]`
- `RecipeId::all_smelting_recipes()` → `&'static [RecipeId]`

These methods are safe to call multiple times per frame with no performance penalty.

### RecipeId Material Detection

The `RecipeId::material()` method returns the `ForgeMaterial` for forge filtering. It uses an exhaustive match on all `RecipeId` variants:

```rust
impl RecipeId {
    pub fn material(&self) -> ForgeMaterial {
        match self {
            RecipeId::CopperIngot | RecipeId::CopperSword | ... => ForgeMaterial::Copper,
            RecipeId::TinIngot | RecipeId::TinSword | ... => ForgeMaterial::Tin,
            RecipeId::BronzeIngot | RecipeId::BronzeSword | ... => ForgeMaterial::Bronze,
            RecipeId::BasicHPPotion => ForgeMaterial::Other,
        }
    }
}
```

### Adding New Recipes

1. Add variant to `variants {}` block in `define_data!` macro
2. **Update `material()` method** - add new variant to appropriate match arm or create new arm
3. The exhaustive match ensures compiler errors if new recipes are added without updating `material()`
