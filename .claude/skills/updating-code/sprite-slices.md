# Typed Sprite Slices

Use typed enums instead of magic strings for sprite slice names.

**File**: `src/assets/sprite_slices.rs`

## Quick Reference

```rust
use crate::assets::{UiAllSlice, UiSelectorsSlice, HealthBarSlice};

// Instead of magic strings:
let cell = sheet.image_node("Slice_10");  // BAD

// Use typed enums:
let cell = sheet.image_node(UiAllSlice::CellBackground.as_str());  // GOOD
```

## Available Enums

| Enum | SpriteSheetKey | Key Variants |
|------|---------------|--------------|
| `UiAllSlice` | `UiAll` | CellBackground, HeartIcon, GoldIcon, TitleBanner, InfoPanelBg, Book, Button* |
| `UiSelectorsSlice` | `UiSelectors` | SelectorFrame1, SelectorFrame2 |
| `HealthBarSlice` | `UiAll` | Health0-Health100, `for_percent(f32)` helper |
| `TravelBookSlice` | `TravelBook` | Banner |
| `BookSlotSlice` | `BookSlot` | Slot |
| `ItemDetailIconsSlice` | (multiple) | AttackIcon, HealthIcon, DefenseIcon, GoldIcon, DefaultStatIcon |
| `DungeonTileSlice` | `DungeonTileset` | FloorTile2-4, TopWall1-4, BottomWall1-4, SideWall2-8, corners, torches, gate |

## Common Patterns

### Grid cell backgrounds
```rust
let cell = sheet.image_node(UiAllSlice::CellBackground.as_str());
```

### Selector animation frames
```rust
let idx1 = sheet.get(UiSelectorsSlice::SelectorFrame1.as_str())?;
let idx2 = sheet.get(UiSelectorsSlice::SelectorFrame2.as_str())?;
```

### Health bar by percentage
```rust
let slice = HealthBarSlice::for_percent(percent);
let idx = sheet.get(slice.as_str())?;
```

### Icons (heart, gold)
```rust
let heart = sheet.image_node(UiAllSlice::HeartIcon.as_str());
let gold = sheet.image_node(UiAllSlice::GoldIcon.as_str());
```

### Stat icons with ItemDetailIconsSlice
See [stat-icons.md](stat-icons.md) for full documentation on `ItemDetailIconsSlice`.

## Adding New Slices

1. Edit `src/assets/sprite_slices.rs`
2. Add variant to appropriate enum
3. Add mapping in `as_str()` method
4. Update documentation

See sprites skill [sprite-slices.md](../sprites/references/sprite-slices.md) for full enum reference.
