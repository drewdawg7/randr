# Typed Sprite Slices

Typed enums for sprite slice names, providing compile-time safety and semantic naming.

**File**: `src/assets/sprite_slices.rs`

## Overview

Instead of magic strings like `"Slice_61"`, use typed enums:

```rust
// Before (error-prone)
let idx = sheet.get("Slice_61")?;

// After (type-safe)
let idx = sheet.get(UiSelectorsSlice::SelectorFrame1.as_str())?;
```

## Benefits

1. **Type safety**: Can't accidentally use wrong slice with wrong sheet
2. **Discoverability**: IDE autocomplete shows available slices
3. **Semantic naming**: `HeartIcon` instead of `Slice_3013`
4. **Grouped by sheet**: Clear which slices belong to which sheet
5. **Helper methods**: e.g., `HealthBarSlice::for_percent(75.0)`

## Enums Reference

### UiAllSlice (SpriteSheetKey::UiAll)

| Variant | Slice Name | Purpose |
|---------|-----------|---------|
| `CellBackground` | Slice_10 | Grid cell background |
| `HeartIcon` | Slice_3013 | Health/HP icon |
| `GoldIcon` | Slice_3019 | Gold/currency icon |
| `TitleBanner` | Slice_3353 | Main menu title |
| `InfoPanelBg` | Slice_2 | Info panel background |
| `Book` | Slice_4891 | Compendium book |
| `ButtonTown` | Slice_295 | Town button (unselected) |
| `ButtonTownSelected` | Slice_329 | Town button (selected) |
| `ButtonProfile` | Slice_193 | Profile button (unselected) |
| `ButtonProfileSelected` | Slice_227 | Profile button (selected) |
| `ButtonQuit` | Slice_397 | Quit button (unselected) |
| `ButtonQuitSelected` | Slice_431 | Quit button (selected) |

### UiSelectorsSlice (SpriteSheetKey::UiSelectors)

| Variant | Slice Name | Purpose |
|---------|-----------|---------|
| `SelectorFrame1` | Slice_61 | Selector animation frame 1 |
| `SelectorFrame2` | Slice_91 | Selector animation frame 2 |

### HealthBarSlice (SpriteSheetKey::UiAll)

Health bar frames from empty to full.

| Variant | Slice Name | Health % |
|---------|-----------|----------|
| `Health0` | Slice_2938 | 0% (empty) |
| `Health10` | Slice_2944 | ~9% |
| `Health20` | Slice_2943 | ~18% |
| `Health30` | Slice_2942 | ~27% |
| `Health40` | Slice_2941 | ~36% |
| `Health50` | Slice_2940 | ~45% |
| `Health60` | Slice_2937 | ~55% |
| `Health70` | Slice_2936 | ~64% |
| `Health80` | Slice_2935 | ~73% |
| `Health90` | Slice_2934 | ~82% |
| `Health100` | Slice_2933 | 100% (full) |

**Helper method**:
```rust
// Get slice for a health percentage
let slice = HealthBarSlice::for_percent(75.0); // Returns Health80
```

### TravelBookSlice (SpriteSheetKey::TravelBook)

| Variant | Slice Name | Purpose |
|---------|-----------|---------|
| `Banner` | banner | Player stats banner background |

### BookSlotSlice (SpriteSheetKey::BookSlot)

| Variant | Slice Name | Purpose |
|---------|-----------|---------|
| `Slot` | slot | Mob display slot in compendium |

## Usage Examples

### Basic usage

```rust
use crate::assets::{GameSprites, SpriteSheetKey, UiAllSlice};

fn spawn_heart(game_sprites: Res<GameSprites>, mut commands: Commands) {
    let Some(sheet) = game_sprites.get(SpriteSheetKey::UiAll) else { return };
    let Some(img) = sheet.image_node(UiAllSlice::HeartIcon.as_str()) else { return };
    commands.spawn((img, Node { width: Val::Px(16.0), height: Val::Px(16.0), ..default() }));
}
```

### Selector animation

```rust
use crate::assets::{UiSelectorsSlice};

let idx1 = sheet.get(UiSelectorsSlice::SelectorFrame1.as_str())?;
let idx2 = sheet.get(UiSelectorsSlice::SelectorFrame2.as_str())?;
let frame_indices = [idx1, idx2];
```

### Health bar with percentage

```rust
use crate::assets::HealthBarSlice;

fn update_health_bar(percent: f32, sheet: &SpriteSheet) {
    let slice = HealthBarSlice::for_percent(percent);
    if let Some(idx) = sheet.get(slice.as_str()) {
        // Update atlas index
    }
}
```

## Adding New Slices

1. Open `src/assets/sprite_slices.rs`
2. Add variant to the appropriate enum
3. Add the string mapping in `as_str()`:

```rust
pub enum UiAllSlice {
    // ... existing variants
    NewSlice,  // Add new variant
}

impl UiAllSlice {
    pub const fn as_str(self) -> &'static str {
        match self {
            // ... existing mappings
            Self::NewSlice => "Slice_1234",  // Add mapping
        }
    }
}
```

4. Update this documentation with the new slice
