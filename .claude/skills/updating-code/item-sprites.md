# Item Sprites

## Overview

Each `ItemId` has a `sprite_name()` method that returns the slice name for its icon in the `IconItems` sprite sheet (`assets/sprites/icon_items.json` + `icon_items.png`).

**File**: `src/item/definitions.rs` — the `sprite_name()` method on `ItemId`

## Sprite Sheet Layout

The `icon_items.png` sprite sheet is 512x2048 pixels containing 32x32 icons in a 16-column, 64-row grid (1024 total cells). Each icon position maps to a named slice (e.g., `Slice_337`).

## Icon File to Slice Name Mapping

The icon source files live at `/Users/drewstewart/Downloads/icons_8.13.20/fullcolor/individual_32x32/iconNNN.png`. To convert an icon file number to a slice name:

```python
# Icon N (1-based) maps to grid position:
col = (N - 1) % 16
row = (N - 1) // 16
x = col * 32
y = row * 32
# Then look up (x, y) in the JSON slices to find the Slice_ name
```

## Current Sprite Assignments

| Item | Slice | Icon Source |
|------|-------|-------------|
| BasicHPPotion | Slice_337 | — |
| Sword | Slice_155 | — |
| Dagger | Slice_156 | — |
| BasicShield | Slice_100 | — |
| CopperHelmet | Slice_101 | — |
| TinSword, CopperSword, BronzeSword | Slice_989 | icon480.png |
| BonkStick | Slice_607 | icon506.png |
| TinHelmet, BronzeHelmet | Slice_108 | icon706.png |
| CopperChestplate, TinChestplate, BronzeChestplate | Slice_551 | icon633.png |
| CopperGauntlets, TinGauntlets, BronzeGauntlets | Slice_558 | icon745.png |
| CopperGreaves, TinGreaves, BronzeGreaves | Slice_367 | icon758.png |
| CopperLeggings, TinLeggings, BronzeLeggings | Slice_42 | icon673.png |
| BronzePickaxe | Slice_826 | icon941.png |
| GoldRing | Slice_50 | icon801.png |
| ImbaRing | Slice_1009 | icon800.png |
| Coal | Slice_693 | icon859.png |
| CopperOre, TinOre | Slice_565 | icon857.png |
| CopperIngot, BronzeIngot | Slice_501 | icon856.png |
| TinIngot | Slice_565 | icon857.png |
| Cowhide | Slice_183 | icon883.png |
| SlimeGel | Slice_952 | icon911.png |
| QualityUpgradeStone | Slice_57 | icon913.png |

## Usage in UI

Item sprites are displayed using the `IconItems` sprite sheet key:

```rust
let sheet = game_sprites.get(SpriteSheetKey::IconItems)?;
let icon = sheet.image_bundle(item_id.sprite_name(), 32.0, 32.0)?;
```

## Adding New Item Sprites

1. Identify the icon file number from the source assets
2. Run the mapping formula above (or check the JSON) to find the `Slice_` name
3. Add the mapping in `ItemId::sprite_name()` in `src/item/definitions.rs`
4. Every `ItemId` variant must be explicitly matched (no wildcard fallback)
