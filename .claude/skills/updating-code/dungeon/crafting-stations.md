# Crafting Stations

Dungeon entities that provide crafting functionality. Currently supports Forge.

## Core Types

### CraftingStationType (`src/crafting_station/mod.rs`)
```rust
pub enum CraftingStationType {
    Forge,
}

impl CraftingStationType {
    pub fn sprite_name(&self) -> &'static str;    // "forge_1_idle"
    pub fn display_name(&self) -> &'static str;   // "Forge"
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

### SpawnTable Method
```rust
SpawnTable::new()
    .forge(1..=1)  // Spawn 1 forge per floor
```

### Where to Add
Add `.forge(count)` to the spawn table in layout files:
- `src/dungeon/layouts/starting_room.rs` - Layout's spawn table (ACTUALLY USED)
- `src/dungeon/floor/definitions.rs` - Floor spec spawn table (NOT used at runtime)

**Important:** Only the spawn table passed to `LayoutBuilder::spawn()` is applied. The `FloorSpec.spawn_table` is currently not applied at runtime.

## Rendering

### Sprite Sheet
- **File:** `assets/sprites/dungeon_entities/forge.png` + `.json`
- **SpriteSheetKey:** `SpriteSheetKey::Forge`
- **Asset path:** `dungeon_entities/forge`

### Visual Size
Crafting stations render at 2x tile size (same as mobs), handled in `src/ui/screens/dungeon/plugin.rs`:
```rust
EntityRenderData::SpriteSheet { sheet_key: SpriteSheetKey::Forge, .. } => {
    (entity_sprite_size, entity_sprite_size)  // 2x tile size
}
```

### Available Slices
| Slice Name | Bounds | Size |
|------------|--------|------|
| `forge_1_idle` | x:16, y:16 | 32x49 |
| `forge_1_active1` | x:16, y:80 | 33x48 |
| `forge_1_active2` | x:79, y:80 | 34x49 |
| `forge_1_active3` | x:208, y:80 | 32x48 |
| `anvil_idle` | x:16, y:432 | 32x16 |
| `anvil_active1-4` | various | 32x32 |

## Behavior

- **Movement:** Blocks player movement (like chests/rocks)
- **Interaction:** Currently none (Space key does nothing)
- **Grid Size:** Always 1x1

## Adding New Crafting Station Types

1. Add variant to `CraftingStationType` in `src/crafting_station/mod.rs`
2. Update `sprite_name()` and `display_name()` methods
3. Add sprite slices to the forge.json (or create new sprite sheet)
4. If new sprite sheet, add `SpriteSheetKey` variant and update rendering in plugin.rs

## Files

| File | Purpose |
|------|---------|
| `src/crafting_station/mod.rs` | CraftingStationType enum |
| `src/dungeon/entity.rs` | DungeonEntity::CraftingStation variant |
| `src/dungeon/spawn.rs` | `.forge()` spawn method |
| `src/assets/sprites.rs` | SpriteSheetKey::Forge |
| `assets/sprites/dungeon_entities/forge.json` | Sprite metadata |
| `assets/sprites/dungeon_entities/forge.png` | Sprite sheet |

## Future Work
- Crafting UI modal on interaction
- Crafting recipes
- Additional station types (Anvil, Enchanting Table)
