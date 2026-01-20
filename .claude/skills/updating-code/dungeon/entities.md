# Dungeon Entities

Entities that can be spawned on dungeon tiles.

## Core Concepts

### Entity Spawning Rules
- Only tiles where `TileType::can_spawn_entity()` returns `true` can have entities
- Currently: `Floor` and `DoorOpen` tiles allow spawning
- `Entrance`, `Exit`, `Wall`, and `Door` do not allow entity spawning

### DungeonEntity Enum (`src/dungeon/entity.rs`)
```rust
pub enum DungeonEntity {
    Chest { variant: u8 },  // 4 visual variants (0-3)
    Mob { mob_id: MobId },  // Any mob type (Goblin, Slime, etc.)
}
```

## Entity Types

### Static Entities (Chest)
- Use `SpriteSheetKey` and `GameSprites` for rendering
- Methods: `sprite_sheet_key()`, `sprite_name()`
- Rendered directly as `ImageNode` in dungeon.rs

### Animated Entities (Mob)
- Use marker component pattern for decoupled rendering
- Spawn with `DungeonMobSprite { mob_id }` marker
- `populate_dungeon_mob_sprites()` system populates sprite + animation
- Reuses `MobSpriteSheets` and `MobAnimation` from mob compendium

## Adding New Mob Types

Mobs that already exist in `MobSpriteSheets` (see `src/ui/mob_animation.rs`) automatically work in dungeons. Just spawn them:

```rust
layout.add_entity(x, y, DungeonEntity::Mob { mob_id: MobId::Dragon });
```

The `DungeonMobSprite` marker and `populate_dungeon_mob_sprites` system handle the rest.

## Adding Static Entity Types

### Step 1: Add Enum Variant
In `src/dungeon/entity.rs`:
```rust
pub enum DungeonEntity {
    Chest { variant: u8 },
    Mob { mob_id: MobId },
    Trap { variant: u8 },  // New static entity
}
```

### Step 2: Add Sprite Assets
Create sprite sheet in `assets/sprites/dungeon_entities/`:
- `trap.png` - The sprite image (16x16)
- `trap.json` - Metadata with frame definitions

### Step 3: Add SpriteSheetKey
In `src/assets/sprites.rs`, add to `SpriteSheetKey` enum and `asset_name()`.

### Step 4: Update Entity Methods
In `src/dungeon/entity.rs`:
```rust
pub fn sprite_sheet_key(&self) -> SpriteSheetKey {
    match self {
        Self::Chest { variant } => ...,
        Self::Mob { .. } => panic!("Mob entities use DungeonMobSprite marker"),
        Self::Trap { .. } => SpriteSheetKey::Trap,
    }
}
```

### Step 5: Update Rendering
In `src/screens/town/tabs/dungeon.rs`, add match arm in entity rendering.

## Entity Rendering Architecture

```
Dungeon Tab (dungeon.rs)           Mob Animation (mob_animation.rs)
┌─────────────────────────┐        ┌──────────────────────────────┐
│ spawn_dungeon_content() │        │ populate_dungeon_mob_sprites │
│   Chest → ImageNode     │        │   DungeonMobSprite marker    │
│   Mob → DungeonMobSprite│───────>│   + MobSpriteSheets lookup   │
│         marker only     │        │   = ImageNode + MobAnimation │
└─────────────────────────┘        └──────────────────────────────┘
       No sprite knowledge              Handles sprite loading
```

### Key Components
- `DungeonMobSprite { mob_id: MobId }` - Marker component in `src/ui/mob_animation.rs`
- `populate_dungeon_mob_sprites()` - System that detects `Added<DungeonMobSprite>` and inserts sprite

## Spawning Entities in Layouts

Example from `layouts/starting_room.rs` with collision prevention:
```rust
use rand::seq::SliceRandom;
use crate::dungeon::DungeonEntity;
use crate::mob::MobId;

// Shuffle spawn points to get random positions without overlap
let mut spawn_points = layout.spawn_points();
spawn_points.shuffle(&mut rng);
let mut spawn_iter = spawn_points.into_iter();

// Spawn entities (each on unique tile)
if let Some((x, y)) = spawn_iter.next() {
    layout.add_entity(x, y, DungeonEntity::Chest { variant: rng.gen_range(0..4) });
}
if let Some((x, y)) = spawn_iter.next() {
    layout.add_entity(x, y, DungeonEntity::Mob { mob_id: MobId::Goblin });
}
if let Some((x, y)) = spawn_iter.next() {
    layout.add_entity(x, y, DungeonEntity::Mob { mob_id: MobId::Slime });
}
```

## Sprite Assets

### Static Sprites
Located in `assets/sprites/dungeon_entities/`:
- `chest_1.png` / `chest_1.json` - Chest variant 1
- `chest_2.png` / `chest_2.json` - Chest variant 2
- `chest_3.png` / `chest_3.json` - Chest variant 3
- `chest_4.png` / `chest_4.json` - Chest variant 4

Source: `2D Pixel Dungeon Asset Pack/items and trap_animation/chest/`

### Animated Mob Sprites
Located in `assets/sprites/mobs/` (shared with combat/compendium):
- `goblin.png` - 27 frames, idle 0-3
- `slime.png` - 18 frames, idle 0-3
- `dragon.png` - 66 frames, idle 0-3

See [mob-sprites.md](../mob-sprites.md) for adding new mob sprites.

## Related
- [mod.md](mod.md) - Dungeon module overview
- [ui.md](ui.md) - Dungeon tab rendering
- [../mob-sprites.md](../mob-sprites.md) - Mob sprite system
