# Mob Sprites

## Overview

Mob sprites are displayed in dungeons, MonsterCompendium (opened with 'b' key), and the results/victory modal. Each `MobId` can have an associated animated sprite sheet with idle and optional death animation support.

## Adding a New Mob Sprite (with Animation)

When adding a new mob sprite, **always include the idle animation if it exists** in the Aseprite file. Include death animation if available.

### 1. Export the Sprite Sheet

Mob sprite sheets are grid-based PNGs exported from Aseprite files that use **slices** (named 32x32 rectangular regions). Export the full grid:

```bash
ASEPRITE="/Users/drewstewart/Library/Application Support/Steam/steamapps/common/Aseprite/Aseprite.app/Contents/MacOS/aseprite"
"$ASEPRITE" --batch "input.aseprite" --save-as assets/sprites/mobs/<mob_name>.png
```

**To analyze slice positions before exporting**, export slices to JSON:
```bash
"$ASEPRITE" --batch "input.aseprite" --list-slices --data /tmp/slices.json --format json-array
cat /tmp/slices.json
```
This outputs slice names and bounds (x, y, w, h) to help determine animation frame indices.

Slices are numbered left-to-right, top-to-bottom. Common row layout:
| Row | Animation | Typical Slices (6-col) |
|-----|-----------|----------------------|
| 0 | Idle | 0-3 |
| 1 | Walk/Run | 6-9 |
| 2 | Attack | 12-17 |
| 3 | Hurt | 18-21 |
| 4 | Death | 24-27 or 30-33 |

### 2. Register the Sprite Sheet in `MobSpriteSheets`

Add the mob to `load_mob_sprite_sheets()` in `src/ui/mob_animation.rs`:

```rust
// <MobName>: <cols>x<rows> grid of 32x32, idle slices <first>-<last>, death slices <first>-<last>
let <mob_name>_texture: Handle<Image> = asset_server.load("sprites/mobs/<mob_name>.png");
let <mob_name>_layout = TextureAtlasLayout::from_grid(UVec2::splat(32), <cols>, <rows>, None, None);
let <mob_name>_layout_handle = layouts.add(<mob_name>_layout);
mob_sheets.insert(
    MobId::<MobName>,
    MobSpriteSheet {
        texture: <mob_name>_texture,
        layout: <mob_name>_layout_handle,
        animation: AnimationConfig {
            first_frame: 0,
            last_frame: 3,
            frame_duration: 0.2,
            looping: true,
        },
        death_animation: Some(AnimationConfig {
            first_frame: 30,  // First slice of death animation
            last_frame: 33,   // Last slice (inclusive)
            frame_duration: 0.15,
            looping: false,   // Play once, stop on last frame
        }),
    },
);
```

### 3. Determine Animation Slice Ranges

Slice indices correspond to grid cells numbered left-to-right, top-to-bottom:
- For a 6-column grid: row N starts at slice `N * 6`
- For an 8-column grid: row N starts at slice `N * 8`
- The **idle** animation is typically row 0 (slices 0-3)
- The **death** animation row varies by sprite pack

### 4. Frame Duration Guidelines

- `0.15` seconds - Death animations (quick, plays once)
- `0.2` seconds - Normal/fast idle (goblin)
- `0.25` seconds - Slower, bouncier idle (slime)
- `0.35` seconds - Large/slow creatures (dragon)

## Key Files

| File | Purpose |
|------|---------|
| `src/ui/animation.rs` | `SpriteAnimation` component, `AnimationConfig`, `animate_sprites()` system |
| `src/ui/mob_animation.rs` | `MobAnimationPlugin`, `AseMobSheets` resource, `AseMobSheet` |
| `src/ui/screens/monster_compendium/render.rs` | `update_compendium_mob_sprite()` - displays animated sprite in MonsterCompendium |
| `assets/sprites/mobs/` | Sprite sheet PNGs and JSON metadata |

## How It Works

### Animation System

1. **Loading**: `load_mob_sprite_sheets()` runs at `PreStartup`, loading aseprite files for each mob
2. **Spawning**: Mobs are spawned with `AseAnimation` component using tag-based animations (idle, hurt, death)
3. **Animating**: `bevy_aseprite_ultra` handles animation playback via `Animation::tag()` with chaining support (`.with_then()`, `.with_repeat()`, `.with_speed()`)

## Currently Supported Mobs

| MobId | Sprite Sheet | Frame Size | Grid | Idle Range | Death Range | Idle Duration |
|-------|-------------|------------|------|------------|-------------|---------------|
| `Goblin` | `goblin.png` | 32x32 | 6x6 | 0-3 | 30-33 | 0.2s |
| `Slime` | `slime.png` | 32x32 | 8x6 | 0-3 | 40-44 | 0.25s |
| `Dragon` | `dragon.png` | 64x32 | 66x1 | 0-3 | None | 0.35s |
| `BlackDragon` | `black_dragon.png` | 64x32 | 16x7 | 2-5 | 98-103 | 0.35s |
| `Merchant` | `merchant.png` | 32x32 | 23x1 | 0-3 | None | 0.15s |
| `DwarfDefender` | `dwarf_defender.png` | 32x32 | 6x7 | 0-3 | 36-41 | 0.2s |
| `DwarfWarrior` | `dwarf_warrior.png` | 32x32 | 6x6 | 0-3 | 30-33 | 0.2s |
| `DwarfMiner` | `dwarf_miner.png` | 32x32 | 6x6 | 0-3 | 30-33 | 0.2s |
| `DwarfKing` | `dwarf_king.png` | 32x32 | 7x7 | 0-3 | 42-48 | 0.25s |

### Non-Square Sprites

For sprites with non-square dimensions (like Dragon at 64x32), use `UVec2::new(width, height)` instead of `UVec2::splat()`:

```rust
let dragon_layout = TextureAtlasLayout::from_grid(UVec2::new(64, 32), 66, 1, None, None);
```

The `frame_size: UVec2` field on `MobSpriteSheet` must match the grid cell dimensions. This is used by the dungeon renderer to compute the correct aspect ratio for the entity node (non-square frames render wider/taller as needed rather than being squished into a square).

### Death Animations

Death animations are stored as `death_animation: Option<AnimationConfig>` in `MobSpriteSheet`. They use `looping: false` to play once and stop on the final frame.

The results modal (`ResultsModalMobSprite`) automatically uses the death animation if available, falling back to idle. Other sprite markers (dungeon, compendium) continue to use the idle animation.

### Sprite Display Sizes

The sprite display containers should be consistent across all locations:

| Location | Container Size | Inner Sprite Size | File |
|----------|----------------|-------------------|------|
| MonsterCompendium | 112x112 | 96x96 | `src/screens/monster_compendium.rs:233-249` |
| Dungeon Tab | 48x48 | 48x48 | `src/screens/town/tabs/dungeon.rs` |

## Special Cases

### CompendiumMobSprite

The `CompendiumMobSprite` in `src/ui/screens/monster_compendium/list.rs` updates dynamically on selection change by inserting a new `AseAnimation` component with the selected mob's aseprite handle and idle tag.
