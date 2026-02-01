# Crafting Stations

Dungeon entities that provide crafting functionality. Supports Forge and Anvil.

## Core Types

### CraftingStationType (`src/crafting_station/mod.rs`)
```rust
pub enum CraftingStationType {
    Forge,
    Anvil,
}

impl CraftingStationType {
    pub fn sprite_name(&self) -> &'static str;    // "forge_1_idle" or "anvil_idle"
    pub fn display_name(&self) -> &'static str;   // "Forge" or "Anvil"
}
```

### Crafting State Components
```rust
// Forge: tracks items in slots
#[derive(Component)]
pub struct ForgeCraftingState {
    pub coal_slot: Option<(ItemId, u32)>,
    pub ore_slot: Option<(ItemId, u32)>,
    pub product_slot: Option<(ItemId, u32)>,
    pub is_crafting: bool,
}

// Anvil: tracks selected recipe
#[derive(Component)]
pub struct AnvilCraftingState {
    pub selected_recipe: Option<RecipeId>,
    pub is_crafting: bool,
}
```

### DungeonEntity Variant (`src/dungeon/entity.rs`)
```rust
DungeonEntity::CraftingStation {
    station_type: CraftingStationType,
    size: GridSize
}
```

## Spawning

### SpawnTable Methods
```rust
SpawnTable::new()
    .forge(1..=1)  // Spawn 1 forge per floor
    .anvil(1..=1)  // Spawn 1 anvil per floor
```

### Where to Add
Add `.forge(count)` or `.anvil(count)` to the spawn table in layout files:
- `src/dungeon/layouts/starting_room.rs` - Layout's spawn table (ACTUALLY USED)
- `src/dungeon/floor/definitions.rs` - Floor spec spawn table (NOT used at runtime)

**Important:** Only the spawn table passed to `LayoutBuilder::spawn()` is applied. The `FloorSpec.spawn_table` is currently not applied at runtime.

## Rendering

### Sprite Sheet
- **File:** `assets/sprites/dungeon_entities/crafting_stations.png` + `.json`
- **SpriteSheetKey:** `SpriteSheetKey::CraftingStations`
- **Asset path:** `dungeon_entities/crafting_stations`

### Visual Size
Handled in `src/ui/screens/dungeon/plugin.rs`:
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

### Available Slices
| Slice Name | Size | Notes |
|------------|------|-------|
| `forge_1_idle` | 32x49 | Forge idle state |
| `forge_1_active1` | 33x48 | Forge animation |
| `forge_1_active2` | 34x49 | Forge animation |
| `forge_1_active3` | 32x48 | Forge animation |
| `anvil_idle` | 32x16 | Anvil idle state |
| `anvil_active1` - `anvil_active_6` | 32x32 | Anvil animation |

## Interaction

When player presses Space near a crafting station:

### Forge
- Opens Forge Modal (see [forge-modal.md](../forge-modal.md))
- Player adds coal and ore to slots
- Closing with filled slots starts smelting (5 seconds)
- Produces ingots in product slot

### Anvil
- Opens Anvil Modal (see [anvil-modal.md](../anvil-modal.md))
- Player browses forging recipes
- Enter on recipe with materials starts crafting (3 seconds)
- Crafted item added directly to inventory

## Animation Timers

Both stations use timer components for crafting animation:
```rust
#[derive(Component)]
pub struct ForgeActiveTimer(pub Timer);  // 5 seconds

#[derive(Component)]
pub struct AnvilActiveTimer(pub Timer);  // 3 seconds
```

When timer expires, `revert_forge_idle` or `revert_anvil_idle` systems in the UI layer:
1. Send crafting complete event (`ForgeCraftingCompleteEvent` or `AnvilCraftingCompleteEvent`)
2. Revert sprite to idle
3. Remove timer component

Game logic handlers in `src/game/crafting_complete.rs` respond to the events:
1. Apply blacksmith skill bonuses
2. Complete crafting (produce output)
3. Grant XP

## Behavior

- **Movement:** Blocks player movement (like chests/rocks)
- **Interaction:** Space key opens modal when adjacent
- **Grid Size:** Always 1x1
- **Can't interact while crafting:** Modal won't open if `is_crafting == true`

## Adding New Crafting Station Types

1. Add variant to `CraftingStationType` in `src/crafting_station/mod.rs`
2. Update `sprite_name()` and `display_name()` methods
3. Add crafting state component (e.g., `NewStationCraftingState`)
4. Add sprite slices to crafting_stations.json (or create new sprite sheet)
5. If new sprite sheet, add `SpriteSheetKey` variant
6. Update size calculation in dungeon plugin if needed
7. Add spawn method to `SpawnTable` (e.g., `.new_station(count)`)
8. Create modal module for interaction UI
9. Add timer component and revert system for animation

## Files

| File | Purpose |
|------|---------|
| `src/crafting_station/mod.rs` | CraftingStationType enum, state components |
| `src/dungeon/entity.rs` | DungeonEntity::CraftingStation variant |
| `src/dungeon/spawn.rs` | `.forge()` and `.anvil()` spawn methods |
| `src/assets/sprites.rs` | SpriteSheetKey::CraftingStations |
| `src/game/crafting_complete.rs` | Crafting completion events and game logic handlers |
| `src/ui/screens/dungeon/plugin.rs` | Rendering, interaction, animation timers |
| `src/ui/screens/forge_modal/` | Forge UI modal |
| `src/ui/screens/anvil_modal/` | Anvil UI modal |
| `assets/sprites/dungeon_entities/crafting_stations.json` | Sprite metadata |
| `assets/sprites/dungeon_entities/crafting_stations.png` | Sprite sheet |
