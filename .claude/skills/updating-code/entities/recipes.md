# Recipe & Crafting System

## Overview

Recipes use the `define_data!` macro for declarative definitions with direct spec lookup via `RecipeId::spec()`.

## Crafting Pattern

`Recipe::craft()` returns `ItemId`. Spawn the item directly:
```rust
let item_id = recipe.craft(&mut player)?;
let item = item_id.spawn();
```

## Key Files

| File | Purpose |
|------|---------|
| `src/item/recipe/definitions.rs` | All recipes defined via `define_data!` macro |
| `src/item/recipe/enums.rs` | `RecipeType`, `ForgeMaterial`, `RecipeError` enums |
| `src/item/recipe/definition.rs` | `Recipe` struct - runtime recipe with crafting logic |

## Recipe Macro System

Recipes use `define_data!` (for static data, not spawnable):
- `RecipeId` enum with all variants
- `RecipeId::spec(&self) -> &'static RecipeSpec` method
- `RecipeId::ALL: &[RecipeId]` for iteration

## Recipe Types

```rust
pub enum RecipeType {
    Smelting,  // ore to ingot (Blacksmith furnace)
    Forging,   // ingots to items (Blacksmith forge)
    Alchemy,   // materials to potions (Alchemist)
}
```

## Adding a New Recipe

Add to `src/item/recipe/definitions.rs` inside the `define_data!` block:

```rust
MyRecipe => RecipeSpec {
    name: "My Item",
    ingredients: HashMap::from([
        (ItemId::BronzeIngot, 4),
    ]),
    output: ItemId::MyItem,
    output_quantity: 1,
    recipe_type: RecipeType::Forging,
},
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
