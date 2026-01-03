# Recipe & Crafting System

## Overview

Recipes define how items are crafted from ingredients. They follow the registry pattern.

## Crafting Pattern

`Recipe::craft()` returns `ItemId` (not `Item`). Callers must spawn the item separately:
```rust
let item_id = recipe.craft(&mut player)?;
let item = game_state().item_registry().spawn(item_id);
```
This keeps the Recipe system decoupled from global state for testability.

## Key Files

| File | Purpose |
|------|---------|
| `src/item/recipe/enums.rs` | `RecipeId`, `RecipeType`, `RecipeError` enums |
| `src/item/recipe/definition.rs` | `Recipe` struct - runtime recipe with crafting logic |
| `src/item/recipe/spec/definition.rs` | `RecipeSpec` struct - static recipe definitions |
| `src/item/recipe/spec/specs.rs` | Static `Lazy<RecipeSpec>` definitions |
| `src/item/recipe/spec/traits.rs` | `RegistryDefaults` impl |

## Recipe Types

```rust
pub enum RecipeType {
    Smelting,  // ore to ingot (Blacksmith furnace)
    Forging,   // ingots to items (Blacksmith forge)
    Alchemy,   // materials to potions (Alchemist)
}
```

## Adding a New Recipe

1. Add variant to `RecipeId` in `src/item/recipe/enums.rs`
2. Update `all_forging_recipes()` or `all_alchemy_recipes()` if applicable
3. Create static spec in `src/item/recipe/spec/specs.rs`:
   ```rust
   pub static MY_RECIPE: Lazy<RecipeSpec> = Lazy::new(|| RecipeSpec {
       name: "My Item",
       ingredients: HashMap::from([
           (ItemId::BronzeIngot, 4),
       ]),
       output: ItemId::MyItem,
       output_quantity: 1,
       recipe_type: RecipeType::Forging,
   });
   ```
4. Import and register in `src/item/recipe/spec/traits.rs`:
   ```rust
   (RecipeId::MyRecipe, MY_RECIPE.clone()),
   ```

## Current Forging Recipes

### Swords (4 ingots each)
- Copper Sword, Tin Sword, Bronze Sword

### Armor (per slot)
| Armor Piece | Ingot Cost |
|-------------|------------|
| Gauntlets | 8 |
| Greaves | 10 |
| Helmet | 12 |
| Leggings | 18 |
| Chestplate | 20 |

Each armor type exists for Copper, Tin, and Bronze metals.

## Recipe Discovery

Helper methods in `RecipeId` impl:
- `all_forging_recipes()` - Returns all forging recipes for UI
- `all_alchemy_recipes()` - Returns all alchemy recipes for UI

## Related Modules

- `src/location/blacksmith/` - Forging and smelting UI/logic
- `src/location/alchemist/` - Alchemy UI/logic
