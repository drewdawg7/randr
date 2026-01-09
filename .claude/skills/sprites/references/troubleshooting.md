# Troubleshooting Sprites

## Sprites Look Blurry

The game uses `ImagePlugin::default_nearest()` for pixel-perfect rendering. If sprites are blurry, verify this is configured in `src/main.rs`.

## JSON Parse Error

Check Aseprite export settings:
- Format must be "Hash" (not "Array")
- File must be valid JSON (no trailing commas)

## Sprite Not Found

1. Check the frame name in Aseprite matches exactly (case-sensitive)
2. Verify both `.png` and `.json` are in `assets/sprites/`
3. Check logs for loading errors

## Sheet Not Loading

The system logs info when sheets load:
```
INFO Loaded sprite sheet 'ui_icons' with 48 sprites
```

If missing, check:
1. Files exist at `assets/sprites/{name}.json` and `assets/sprites/{name}.png`
2. JSON is valid (try opening in a JSON validator)
3. Run with `RUST_LOG=debug` to see detailed errors

## Concepts

### Texture Atlas
- `TextureAtlasLayout` - defines where sprites are in the sheet
- `TextureAtlas` - component that references the layout + current sprite index
- The game's `SpriteSheet` wraps these with named access

### Pixel-Perfect Rendering
The game uses nearest-neighbor filtering (`ImagePlugin::default_nearest()`) instead of bilinear interpolation to keep pixel art crisp when scaled.
