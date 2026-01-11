# Aseprite Workflow

## CLI Export (Batch Processing)

Aseprite CLI enables automated sprite extraction without opening the GUI.

**Path (Steam version):**
```bash
ASEPRITE="/Users/drewstewart/Library/Application Support/Steam/steamapps/common/Aseprite/Aseprite.app/Contents/MacOS/aseprite"
```

### Extract Single Sprite
```bash
"$ASEPRITE" --batch input.aseprite --slice "Banner" --save-as banner.png
```

### Export with JSON Metadata
```bash
"$ASEPRITE" --batch input.aseprite \
  --sheet output.png \
  --data output.json \
  --format json-hash
```

### Export All Slices as Separate Files
```bash
"$ASEPRITE" --batch input.aseprite --split-slices --save-as "{slice}.png"
```

### List Layers
```bash
"$ASEPRITE" --batch input.aseprite --list-layers
```

### Common Options
| Option | Description |
|--------|-------------|
| `--batch` | Run without GUI (required for CLI) |
| `--slice <name>` | Crop to named slice |
| `--split-slices` | Export each slice separately |
| `--data <file.json>` | Output JSON metadata |
| `--sheet <file.png>` | Output sprite sheet image |
| `--format json-hash` | Use hash format for JSON |
| `--list-layers` | Print layer names |

Run `"$ASEPRITE" --help` for full options.

---

## GUI Workflow

### Grid-Aligned Sprites (e.g., 16x16 icons)

1. **Open** the PNG in Aseprite
2. **Import**: `File > Import Sprite Sheet`
   - Type: "By Cell Size" or "By Rows and Columns"
   - Cell size: 16x16 (or your grid size)
3. **Rename** frames in the timeline (double-click frame label)
4. **Export**: `File > Export Sprite Sheet`
   - Sheet Type: Keep original layout
   - Check: "JSON Data"
   - JSON Format: "Hash"

## Irregular Sprites (mixed sizes)

1. **Open** the PNG in Aseprite
2. Use **Slices** (`Frame > Slices > New Slice`) to define regions
3. **Name** each slice descriptively
4. **Export** with "Slices" checked in the export dialog

## JSON Format: Frames

```json
{
  "frames": {
    "heart_full": {
      "frame": {"x": 0, "y": 0, "w": 16, "h": 16}
    },
    "heart_half": {
      "frame": {"x": 16, "y": 0, "w": 16, "h": 16}
    }
  },
  "meta": {
    "size": {"w": 48, "h": 16}
  }
}
```

## JSON Format: Slices

```json
{
  "frames": {
    "UI_ALL.aseprite": {
      "frame": {"x": 0, "y": 0, "w": 2048, "h": 2576}
    }
  },
  "meta": {
    "size": {"w": 2048, "h": 2576},
    "slices": [
      {
        "name": "heart_empty",
        "keys": [{"frame": 0, "bounds": {"x": 17, "y": 993, "w": 14, "h": 14}}]
      }
    ]
  }
}
```

Both frames and slices are loaded into `SpriteSheet.sprites` and accessible by name.

## Finding Slice Dimensions

```bash
grep -A 1 '"Slice_193"\|"heart_empty"' assets/sprites/ui_all.json
```

Example output:
```json
{ "name": "Slice_193", "keys": [{ "frame": 0, "bounds": {"x": 1632, "y": 337, "w": 47, "h": 14 } }] },
```

The `bounds` field gives `w` (width) and `h` (height). For UI scaling:
- Menu buttons are typically 47x14 pixels
- Scale 3x for readable size: 141x42 pixels in the Node
