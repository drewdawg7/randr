# Mob Sprites

## Overview

Mob sprites are displayed during combat on the fight screen. Each `MobId` can have an associated sprite that is shown when fighting that mob.

## Adding a New Mob Sprite

1. **Export the sprite** from Aseprite to `assets/sprites/mobs/<mob_name>.png`:
   ```bash
   ASEPRITE="/Users/drewstewart/Library/Application Support/Steam/steamapps/common/Aseprite/Aseprite.app/Contents/MacOS/aseprite"
   "$ASEPRITE" --batch "input.aseprite" --frame-range 0,0 --save-as assets/sprites/mobs/<mob_name>.png
   ```

2. **Add the field** to `SpriteAssets` in `src/assets/sprites.rs`:
   ```rust
   // Mob sprites
   pub mob_slime: Option<Handle<Image>>,
   pub mob_goblin: Option<Handle<Image>>,
   pub mob_<new_mob>: Option<Handle<Image>>,  // Add new mob here
   ```

3. **Load the sprite** in the `load_assets` function in `src/assets/sprites.rs`:
   ```rust
   game_assets.sprites.mob_<new_mob> = try_load(&asset_server, "sprites/mobs/<new_mob>.png");
   ```

4. **Update `mob_sprite()`** method in `SpriteAssets` to return the sprite for the new `MobId`:
   ```rust
   pub fn mob_sprite(&self, mob_id: MobId) -> Option<&Handle<Image>> {
       match mob_id {
           MobId::Slime => self.mob_slime.as_ref(),
           MobId::Goblin => self.mob_goblin.as_ref(),
           MobId::<NewMob> => self.mob_<new_mob>.as_ref(),  // Add new mob here
           _ => None,
       }
   }
   ```

## Key Files

- `src/assets/sprites.rs` - `SpriteAssets` struct, `mob_sprite()` method, `load_assets()` function
- `src/screens/fight/ui.rs` - `populate_mob_sprite()` system that displays the sprite
- `assets/sprites/mobs/` - Directory containing mob sprite PNGs

## How It Works

The `populate_mob_sprite` system in `src/screens/fight/ui.rs`:
1. Gets the current combat from `ActiveCombatResource`
2. Looks up the mob's `mob_id` from `combat.mob.mob_id`
3. Calls `game_assets.sprites.mob_sprite(mob_id)` to get the correct sprite
4. Inserts the sprite as an `ImageNode` on entities with `NeedsMobSprite` marker

Mobs without a sprite will show nothing (the system returns early if `mob_sprite()` returns `None`).
