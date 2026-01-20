# Stat Icons

Icons for displaying item stats in the store detail panel and elsewhere.

**Files:**
- `src/assets/sprite_slices.rs` - `ItemDetailIconsSlice` enum
- `assets/sprites/item_detail_icons/` - Icon PNG and JSON files

## ItemDetailIconsSlice Enum

Maps stat types to icons using separate sprite sheets per icon.

```rust
pub enum ItemDetailIconsSlice {
    AttackIcon,      // attack_icon.png
    HealthIcon,      // Icon_03_Outline.png
    DefenseIcon,     // Icon_10_Outline.png
    GoldIcon,        // Icon_05_Outline.png
    DefaultStatIcon, // Icon_08_Outline.png
}
```

### Key Methods

- `for_stat(stat_type: StatType) -> Self` - Maps StatType to icon
- `sprite_sheet_key() -> SpriteSheetKey` - Returns the sheet for this icon
- `as_str() -> &'static str` - Returns the slice name within the sheet

## Usage Pattern

**Within a system that has `Res<GameSprites>`:**

```rust
// Get the icon slice for a stat type
let icon_slice = ItemDetailIconsSlice::for_stat(stat_type);

// Use sprite_sheet_key() to get the correct sheet
if let Some(sheet) = game_sprites.get(icon_slice.sprite_sheet_key()) {
    if let Some(img) = sheet.image_node(icon_slice.as_str()) {
        row.spawn((img, Node::default()));
    }
}
```

## Icon Mappings

| StatType | ItemDetailIconsSlice | Source File |
|----------|---------------------|-------------|
| Health | HealthIcon | Icon_03_Outline.png |
| Attack | AttackIcon | attack_icon.png |
| Defense | DefenseIcon | Icon_10_Outline.png |
| GoldFind | DefaultStatIcon | Icon_08_Outline.png |
| Mining | DefaultStatIcon | Icon_08_Outline.png |
| MagicFind | DefaultStatIcon | Icon_08_Outline.png |
| (price) | GoldIcon | Icon_05_Outline.png |

## Adding New Stat Icons

1. Add PNG to `assets/sprites/item_detail_icons/`
2. Create JSON file with frame definition
3. Add `SpriteSheetKey` variant in `src/assets/sprites.rs`
4. Add `ItemDetailIconsSlice` variant in `src/assets/sprite_slices.rs`
5. Update `sprite_sheet_key()` and `for_stat()` matches
