# Batch Sprite Processing

## Overview

Workflow for converting PNG sprite sheet packs into sliced Aseprite files. Useful when importing third-party sprite packs that contain multiple sprite sheets (e.g., Minifolks, MiniMonsters).

## Prerequisites

- **Aseprite CLI**: `/Users/drewstewart/Library/Application Support/Steam/steamapps/common/Aseprite/Aseprite.app/Contents/MacOS/aseprite`
- **unar**: For extracting .rar/.7z archives (`brew install unar`)

## Extracting Sprite Packs

Most sprite packs come as .rar or .7z archives with this structure:

```
PackName/
├── Aseprite/          (optional, source files)
├── Outline/           (sprites with outline)
└── Without Outline/   (sprites without outline)
```

Extract with `unar`:

```bash
unar -o /tmp/sprite_extraction "PackName.rar"
```

The "Without Outline" folder naming varies (`Without outline`, `Without Outline`). Use case-insensitive matching:

```bash
find /tmp/sprite_extraction -ipath "*/without outline*" -type f -name "*.png"
```

## Batch Slicing with Aseprite CLI

### Standard 32x32 Sprites

Create named slices for each 32x32 cell in the sprite sheet:

```bash
ASEPRITE="/Users/drewstewart/Library/Application Support/Steam/steamapps/common/Aseprite/Aseprite.app/Contents/MacOS/aseprite"

cat > /tmp/slice_sheet.lua << 'EOF'
local input = app.params["input"]
local output = app.params["output"]

local spr = app.open(input)
if spr then
  local cols = math.floor(spr.width / 32)
  local rows = math.floor(spr.height / 32)
  local idx = 0
  for row = 0, rows - 1 do
    for col = 0, cols - 1 do
      local slice = spr:newSlice(Rectangle(col * 32, row * 32, 32, 32))
      slice.name = "Slice_" .. idx
      idx = idx + 1
    end
  end
  spr:saveCopyAs(output)
  spr:close()
end
EOF

"$ASEPRITE" --batch \
  --script-param "input=source.png" \
  --script-param "output=output.aseprite" \
  --script /tmp/slice_sheet.lua
```

### Non-Standard Sizes (Projectiles, Small Sprites)

For sprites smaller than 32x32, use the shortest dimension as the cell size:

```lua
local cell = math.min(spr.width, spr.height)
local cols = math.floor(spr.width / cell)
local rows = math.floor(spr.height / cell)
```

Common cases:
- **16x16** (single projectile): 1 slice
- **64x16** (projectile animation): 4 slices of 16x16
- **24x24** (small item): 1 slice

### Batch Processing All Files

```bash
for f in /path/to/sprites/*.png; do
  name=$(basename "$f" .png)
  "$ASEPRITE" --batch \
    --script-param "input=$f" \
    --script-param "output=/path/to/output/${name}.aseprite" \
    --script /tmp/slice_sheet.lua
done
```

## Sprite Sheet Layouts

These sprite packs typically use grid layouts where each row is a different animation:

| Row | Animation |
|-----|-----------|
| 0 | Idle |
| 1 | Walk/Run |
| 2 | Attack |
| 3 | Hurt |
| 4 | Death |
| ... | Varies by pack |

The number of columns varies per animation (e.g., idle might be 4 frames, attack might be 6).

## Output

Sliced Aseprite files contain:
- The original sprite sheet image (unchanged)
- Named slices (`Slice_0`, `Slice_1`, ...) for each cell, numbered left-to-right, top-to-bottom
- Can be exported via Aseprite CLI with `--split-slices` or used with `--slice "Slice_N"`

## Notes

- macOS `mach_vm_read` warnings during batch processing are harmless diagnostics
- Check for non-32-aligned sprites before batch processing: any sprite with width or height not divisible by 32 needs the native-size approach
- Verify filename uniqueness before copying sprites from multiple packs into one folder
