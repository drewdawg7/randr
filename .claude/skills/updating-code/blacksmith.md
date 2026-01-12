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

## Recipe System

### Location
`src/item/recipe/specs.rs` - Recipe definitions using `entity_macros::define_data!`

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
