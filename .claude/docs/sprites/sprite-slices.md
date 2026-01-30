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
| `ShopBgSlice` | `ShopBgSlices` | TopLeft, TopCenter, TopRight, MiddleLeft, Center, MiddleRight, BottomLeft, BottomCenter, BottomRight (implements `NineSlice`) |
| `DetailPanelSlice` | `DetailPanelBg` | TopLeft, TopCenter, TopRight, MiddleLeft, Center, MiddleRight, BottomLeft, BottomCenter, BottomRight (implements `NineSlice`) |
| `FightBannerSlice` | `FightBannerSlices` | Left, Center, Right (implements `ThreeSlice`) |

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

## NineSlice Trait

The `NineSlice` trait enables generic nine-slice panel spawning. Implemented by `ShopBgSlice` and `DetailPanelSlice`.

```rust
use crate::assets::{NineSlice, ShopBgSlice};
use crate::ui::widgets::spawn_nine_slice_panel;

// Spawn a nine-slice panel
spawn_nine_slice_panel::<ShopBgSlice>(parent, &game_sprites, width, height);
```

See [nine_slice.md](../widgets/nine_slice.md) for full documentation.

## ThreeSlice Trait

The `ThreeSlice` trait enables generic 3-slice horizontal banner spawning. Implemented by `FightBannerSlice`.

```rust
use crate::assets::{ThreeSlice, FightBannerSlice};
use crate::ui::widgets::spawn_three_slice_banner;

// Spawn a three-slice banner
spawn_three_slice_banner::<FightBannerSlice>(parent, &game_sprites, width);
```

See [three_slice.md](../widgets/three_slice.md) for full documentation.

## Adding New Slices

1. Edit `src/assets/sprite_slices.rs`
2. Add variant to appropriate enum
3. Add mapping in `as_str()` method
4. Update documentation

See sprites skill for full enum reference.
