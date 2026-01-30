# Item Sprites

## Overview

Each `ItemId` has sprite metadata defined inline in the `define_entity!` macro using the `@sprite` annotation. The macro generates `sprite_name()` and `sprite_sheet_key()` methods.

**File**: `src/item/definitions.rs`

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

## Sprite Annotation Syntax

Items define their sprite inline with the `@sprite` annotation:

```rust
Sword {
    name: String::from("Sword"),
    item_type: ItemType::Equipment(EquipmentType::Weapon),
    // ... other fields ...
    @sprite: "Slice_155",  // Uses default sheet (IconItems)
}

GoldSword {
    name: String::from("Gold Sword"),
    // ...
    @sprite: "gold_sword" in SpriteSheetKey::GoldSword,  // Custom sheet
}
```

The default sprite sheet (`SpriteSheetKey::IconItems`) is set in the `sprites(...)` block:

```rust
sprites(default_sheet: SpriteSheetKey::IconItems);
```

## Usage in UI

Item sprites are displayed using the generated methods:

```rust
let sheet = game_sprites.get(item_id.sprite_sheet_key())?;
let icon = sheet.image_bundle(item_id.sprite_name(), 32.0, 32.0)?;
```

## Adding New Item Sprites

1. Identify the icon file number from the source assets
2. Run the mapping formula above (or check the JSON) to find the `Slice_` name
3. Add the `@sprite` annotation to the item variant in `src/item/definitions.rs`
   - Use `@sprite: "slice_name"` for items using the default `IconItems` sheet
   - Use `@sprite: "slice_name" in SpriteSheetKey::SheetName` for custom sheets
