# GameSprites Resource

## Overview

All sprites in this codebase are accessed through `GameSprites`, which stores sprite sheets indexed by `SpriteSheetKey`. **Never use `GameAssets` for sprites** - that's legacy code for standalone images only.

## Architecture

```rust
// GameSprites uses a HashMap, NOT individual fields
#[derive(Resource, Default)]
pub struct GameSprites {
    sheets: HashMap<SpriteSheetKey, SpriteSheet>,
}

impl GameSprites {
    pub fn get(&self, key: SpriteSheetKey) -> Option<&SpriteSheet>;
}

// All sprite sheets are identified by this enum
pub enum SpriteSheetKey {
    UiIcons,
    UiButtons,
    BookUi,
    UiFrames,
    UiBars,
    UiAll,
    IconItems,
    UiSelectors,
    TravelBook,
    BookSlot,
    // Add new variants here
}
```

## Accessing Sprites

```rust
// CORRECT: Use SpriteSheetKey enum
let Some(ui_all) = game_sprites.get(SpriteSheetKey::UiAll) else { return };
let Some(idx) = ui_all.get("Slice_4891") else { return };

// WRONG: GameSprites doesn't have direct fields
// game_sprites.ui_all  // This doesn't exist!
```

## Adding a New Sprite Sheet

When adding a new sprite (e.g., from an external asset pack):

### Step 1: Create the sprite sheet files

```
assets/sprites/
├── my_sprite.png    # The sprite image
└── my_sprite.json   # Metadata file
```

JSON format for a single sprite:
```json
{
  "frames": {
    "sprite_name": { "frame": {"x": 0, "y": 0, "w": 30, "h": 30} }
  },
  "meta": { "size": {"w": 30, "h": 30} }
}
```

### Step 2: Add SpriteSheetKey variant

In `src/assets/sprites.rs`:

```rust
pub enum SpriteSheetKey {
    // ... existing variants ...
    MySprite,  // Add new variant
}

impl SpriteSheetKey {
    pub const fn all() -> &'static [Self] {
        &[
            // ... existing ...
            Self::MySprite,  // Add to array
        ]
    }

    pub const fn asset_name(&self) -> &'static str {
        match self {
            // ... existing ...
            Self::MySprite => "my_sprite",  // Matches filename without extension
        }
    }
}
```

That's it! The sprite sheet loads automatically via `SpriteSheetKey::all()`.

### Step 3: Use in code

```rust
fn my_system(game_sprites: Res<GameSprites>) {
    let Some(sheet) = game_sprites.get(SpriteSheetKey::MySprite) else { return };
    let Some(idx) = sheet.get("sprite_name") else { return };
    // Use idx with ImageNode::from_atlas_image()
}
```

## Spawn Functions Must Be Systems

**WRONG**: Passing GameSprites to helper functions
```rust
// DON'T DO THIS
fn handle_toggle(mut commands: Commands, game_sprites: Res<GameSprites>) {
    spawn_my_ui(&mut commands, &game_sprites);  // Bad!
}

fn spawn_my_ui(commands: &mut Commands, game_sprites: &GameSprites) {
    // ...
}
```

**CORRECT**: Make spawn functions into systems with trigger resources
```rust
#[derive(Resource)]
struct SpawnMyUI;

fn handle_toggle(mut commands: Commands) {
    commands.insert_resource(SpawnMyUI);
}

fn spawn_my_ui(mut commands: Commands, game_sprites: Res<GameSprites>) {
    commands.remove_resource::<SpawnMyUI>();
    // Access sprites directly as system parameter
}

// In plugin:
app.add_systems(Update, spawn_my_ui.run_if(resource_exists::<SpawnMyUI>));
```

## Common Mistakes

1. **Using GameAssets for sprites** - GameAssets is legacy; use GameSprites
2. **Passing GameSprites to functions** - Make them systems instead
3. **Modifying existing sprite sheets** - Create new sprite sheets for new sprites
4. **Direct field access** - Use `game_sprites.get(SpriteSheetKey::X)`, not `game_sprites.x`
