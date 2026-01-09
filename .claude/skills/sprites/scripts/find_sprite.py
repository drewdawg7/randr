#!/usr/bin/env python3
"""Look up sprite dimensions from a sprite sheet JSON file."""

import json
import sys
from pathlib import Path


def find_sprite(sheet_name: str, sprite_name: str) -> str:
    """Find a sprite in a sheet and return its dimensions."""
    # Find the JSON file
    json_path = Path(f"assets/sprites/{sheet_name}.json")
    if not json_path.exists():
        return f"Error: {json_path} not found"

    with open(json_path) as f:
        data = json.load(f)

    # Check frames (regular sprites)
    if "frames" in data:
        for name, frame_data in data["frames"].items():
            if sprite_name.lower() in name.lower():
                frame = frame_data["frame"]
                return f"{name}: {frame['w']}x{frame['h']} at ({frame['x']}, {frame['y']})"

    # Check slices (irregular regions)
    if "meta" in data and "slices" in data["meta"]:
        for slice_data in data["meta"]["slices"]:
            if sprite_name.lower() in slice_data["name"].lower():
                if slice_data["keys"]:
                    bounds = slice_data["keys"][0]["bounds"]
                    return f"{slice_data['name']}: {bounds['w']}x{bounds['h']} at ({bounds['x']}, {bounds['y']})"

    return f"Error: '{sprite_name}' not found in {sheet_name}"


def main():
    if len(sys.argv) != 3:
        print("Usage: find_sprite.py <sheet_name> <sprite_name>")
        print("Example: find_sprite.py ui_all Slice_3353")
        sys.exit(1)

    sheet_name = sys.argv[1]
    sprite_name = sys.argv[2]
    print(find_sprite(sheet_name, sprite_name))


if __name__ == "__main__":
    main()
