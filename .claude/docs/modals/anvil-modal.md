# Anvil Modal

The anvil modal allows players to craft equipment (swords, armor) from ingots using forging recipes.

## Overview

- Opens when player interacts with an anvil in a dungeon (Space key)
- Left side: Recipe grid showing all forging recipes
- Right side: Player inventory grid (5x5)
- Far right: Item detail pane showing recipe ingredients or selected item info
- Tab switches focus between recipe grid and inventory
- Enter on a recipe with sufficient materials starts crafting

## Key Files

| File | Purpose |
|------|---------|
| `src/ui/screens/anvil_modal/mod.rs` | Module exports |
| `src/ui/screens/anvil_modal/state.rs` | State types and `RegisteredModal` impl |
| `src/ui/screens/anvil_modal/render.rs` | UI spawning and detail pane population |
| `src/ui/screens/anvil_modal/input.rs` | Navigation and crafting logic |
| `src/ui/screens/anvil_modal/plugin.rs` | System registration |
| `src/crafting_station/mod.rs` | `AnvilCraftingState` component |

## State Types

```rust
// Modal root marker
pub struct AnvilModalRoot;

// Grid markers
pub struct AnvilRecipeGrid;   // Shows all forging recipes
pub struct AnvilPlayerGrid;   // 5x5 inventory

// Modal state resource
pub struct AnvilModalState {
    pub recipes_focused: bool,  // false = recipe grid, true = inventory
}

// Tracks which anvil entity is open
pub struct ActiveAnvilEntity(pub Entity);

// Trigger resources
pub struct SpawnAnvilModal;
pub struct AnvilRecipeRefresh;       // Triggers recipe grid update
pub struct CloseAnvilForCrafting;    // Triggers close with animation
```

## AnvilCraftingState Component

Attached to anvil entities to track crafting state:

```rust
#[derive(Component, Default, Clone)]
pub struct AnvilCraftingState {
    pub selected_recipe: Option<RecipeId>,
    pub is_crafting: bool,
}

impl AnvilCraftingState {
    pub fn complete_crafting(&mut self) -> Option<RecipeId>;  // Returns recipe and resets state
}
```

## Crafting Flow

1. Player opens anvil modal (Space near anvil)
2. Browse recipe grid - recipes show grayed out if player lacks materials
3. Select recipe, press Enter
4. If player has all required ingredients:
   - Ingredients consumed from inventory
   - Modal closes
   - Anvil animation plays for 3 seconds
5. On animation complete: crafted item added to inventory
6. Reopen modal to craft more items

## Recipe Display

Recipes come from `RecipeId::all_forging_recipes()` which returns all recipes with `recipe_type == RecipeType::Forging`.

Each recipe entry shows:
- Output item icon
- Grayed out if player lacks required ingredients

## Key Systems

### `spawn_anvil_modal`
Builds the modal UI with recipe grid, inventory grid, and detail pane.

### `handle_anvil_modal_tab`
Toggles `recipes_focused` and updates `ItemGrid.is_focused`.

### `handle_anvil_modal_navigation`
- Recipe grid focused: Arrow keys navigate recipes
- Inventory focused: Arrow keys navigate inventory grid

### `handle_anvil_modal_select`
Handles Enter key when recipe focused:
- Checks if player has all required ingredients
- If yes: consumes ingredients, sets anvil to crafting state, triggers close
- If no: does nothing

### `refresh_anvil_recipes`
Updates recipe grid entries when `AnvilRecipeRefresh` resource exists.

### `update_anvil_detail_pane_source`
Updates `pane.source` based on focus and grid selection. Only runs when:
- `FocusState` changes (tab between recipe grid and inventory)
- `ItemGrid.selected_index` changes (navigation)

### `populate_anvil_detail_pane_content`
Renders content when source or inventory changes. Only runs when:
- `pane.source` changed (via source update system)
- `inventory.is_changed()` (ingredient counts may have changed)

Shows:
- When recipe focused: recipe name, required ingredients (with have/need counts), output stats
- When inventory focused: selected item info (name, type, quality, quantity, stats)

### `handle_anvil_close_with_crafting`
When `CloseAnvilForCrafting` resource exists:
- Starts anvil animation (`anvil_active1` through `anvil_active_6`)
- Adds `AnvilActiveTimer(3 seconds)`
- Closes modal

### `revert_anvil_idle` (in dungeon plugin)
When timer expires:
- Calls `anvil_state.complete_crafting()`
- Creates output item and adds to player inventory
- Reverts to idle sprite

## Opening the Modal

From dungeon interaction (`handle_mine_interaction`):

```rust
// Check anvil isn't already crafting
if anvil_state.map(|s| s.is_crafting).unwrap_or(false) {
    return; // Can't open while crafting
}

commands.insert_resource(ActiveAnvilEntity(entity_id));
commands.insert_resource(AnvilModalState::default());
commands.insert_resource(SpawnAnvilModal);
```

## Sprite Assets

Anvil sprites are in `SpriteSheetKey::CraftingStations`:
- `anvil_idle` - idle sprite (32x16)
- `anvil_active1` through `anvil_active_6` - crafting animation frames (32x32)

## Sprite Rendering

Anvil renders at 2x tile width and 1x tile height to match its 2:1 aspect ratio:

```rust
EntityRenderData::SpriteSheet { sheet_key: SpriteSheetKey::CraftingStations, sprite_name } => {
    if sprite_name.starts_with("anvil") {
        // Anvil is 32x16 (2:1 aspect) - render at 2x tile width, 1x tile height
        (entity_sprite_size, tile_size)
    } else {
        // Forge is 32x49 - render at 2x tile size
        (entity_sprite_size, entity_sprite_size)
    }
}
```

## Differences from Forge Modal

| Aspect | Forge | Anvil |
|--------|-------|-------|
| Input method | Drag items to slots | Select recipe from grid |
| Crafting trigger | Close modal with slots filled | Press Enter on recipe |
| Output location | Product slot on forge | Directly to inventory |
| Recipe source | Fixed (coal + ore = ingot) | `RecipeId::all_forging_recipes()` |
| Animation duration | 5 seconds | 3 seconds |
| Sprite dimensions | 32x49 | 32x16 |
