# Dungeon System

## Overview
The dungeon system provides procedurally generated dungeon exploration with monster and chest rooms on a 5x5 grid.

## Key Files

| File | Purpose |
|------|---------|
| `src/dungeon/definition.rs` | Dungeon and DungeonRoom structs, core methods |
| `src/dungeon/generation.rs` | Procedural generation algorithm |
| `src/dungeon/enums.rs` | Direction, DungeonError, RoomType enums |
| `src/dungeon/tests.rs` | Comprehensive unit and integration tests |
| `src/system.rs` | GameState dungeon field and CombatSource enum |
| `src/ui/components/dungeon/tab.rs` | Town tab entry point (DungeonTab) |
| `src/ui/components/dungeon/minimap.rs` | Minimap rendering with fog of war |
| `src/ui/components/dungeon/campfire_art.rs` | Animated campfire ASCII art for rest rooms |
| `src/ui/components/screens/dungeon.rs` | Main dungeon gameplay screen |
| `src/ui/components/screens/fight.rs` | Combat integration (CombatSource handling) |

## Architecture

### Dungeon Storage
- Dungeon is stored on `GameState.dungeon: Option<Dungeon>` (NOT on Town)
- Persists across screen switches until explicitly reset
- Generated on first entry via `game_state().enter_dungeon()`

### Screen Flow
```
Town (DungeonTab) -> Id::Dungeon (DungeonScreen) -> Id::Fight -> Id::Dungeon
```

### DungeonScreen State Machine
```rust
enum DungeonState {
    RoomEntry,    // Show room type + action (Fight/Open)
    Navigation,   // Show available directions after room cleared
    RestRoom,     // Rest room UI with heal option
}

enum CompassPosition {
    North, East, South, West,
    Center,  // Leave Dungeon
}
```

**State transitions:**
- `RoomEntry` -> `Navigation`: When `room.is_cleared && state == RoomEntry` (checked in `view()`)
- `RoomEntry` -> `RestRoom`: When entering a Rest room type
- `RestRoom` -> `Navigation`: After selecting "Continue" option
- `Navigation` -> `RoomEntry`: After `move_player()` to new room

### Navigation UI (Compass Layout)
The navigation screen uses a compass-style grid layout:
```
         [North]
[West]   [Leave]   [East]
         [South]
```

**Key implementation details:**
- `compass_selection: CompassPosition` tracks current selection
- `compass_move(CmdDirection)` handles grid navigation between positions
- Arrow keys navigate (up/down/left/right), Enter confirms
- Selection defaults to Center (Leave) after state transitions
- Only available directions are shown
- Cleared rooms display checkmark indicator

**Rendering methods:**
- `render_navigation()` - Main compass layout
- `render_compass_button()` - Direction buttons with cleared status
- `render_leave_button()` - Center leave option

### Combat Integration
- `CombatSource` enum tracks origin: `Field` or `Dungeon`
- Set in `handle_room_entry_submit()` before starting combat
- Checked in `FightScreen` to:
  - Hide "Fight Again" option for dungeon combat
  - Return to correct screen after combat

**Critical:** `FightScreen.reset_for_new_combat()` must be called when leaving combat to reset `victory_processed`. Otherwise, subsequent fights won't mark rooms as cleared.

## Generation Algorithm
Located in `src/dungeon/generation.rs`:
1. Start at random edge position
2. Corridor-favoring random walk (70% chance to continue same direction)
3. Room types: 60% Monster, 25% Chest, 15% Rest
4. Entry room is always Monster type
5. Guarantees at least 1 Chest and 1 Rest room per dungeon

### Corridor-Favoring Walk
- 70% chance to continue in same direction
- 30% chance to pick random direction
- When stuck, jumps to random existing room and tries new direction
- Results in more corridor-like layouts vs clustered rooms

### Sparsity Constants
- `MAX_FILL_PERCENT` in `definition.rs`: 0.50 (50% of 5x5 grid = max 12 rooms)
- `MIN_ROOMS` in `generation.rs`: 5

### Room Type Guarantees
`ensure_room_type()` converts Monster rooms if needed:
- Called after generation for Chest and Rest types
- Excludes entry room from conversion
- Rest rooms are pre-cleared by default

## Key Methods

### Dungeon
- `generate()` - Creates room layout
- `spawn_mob()` - Weighted random mob from `mob_table`
- `move_player(Direction)` - Update player position
- `available_directions()` - Get valid movement options
- `current_room()` / `current_room_mut()` - Get room at player position

### DungeonRoom
- `visit()` - Mark room as visited (also sets revealed)
- `reveal()` - Mark room as revealed (visible on map but not visited)
- `clear()` - Mark room as cleared
- `open_chest()` - Get loot drops (for Chest rooms)

### GameState
- `enter_dungeon()` - Generate if needed, switch to dungeon screen
- `leave_dungeon()` - Return to town
- `reset_dungeon()` - Clear current dungeon

## Common Patterns

### Handling Dungeon Combat Return
In `FightScreen.return_from_combat()`:
```rust
match gs.combat_source {
    CombatSource::Dungeon => gs.current_screen = Id::Dungeon,
    CombatSource::Field => gs.current_screen = Id::Town,
}
gs.combat_source = CombatSource::default();
self.reset_for_new_combat();  // Critical!
```

### Checking Room Cleared State
In `DungeonScreen.view()`:
```rust
if room.is_cleared && self.state == DungeonState::RoomEntry {
    self.state = DungeonState::Navigation;
    self.reset_selection();
}
```

## UI Integration
- Town screen: Added `DungeonTab` with `BorderTheme::Stone`
- Dungeon icon: `\u{F0509}` (castle icon)
- Uses `LIGHT_STONE` and `GRANITE` colors for theme

### Visual Theme (Stone Background & Border)
Both `DungeonTab` (entrance screen) and `DungeonScreen` (inside dungeon) use:
- **Background**: `MINE_BG` fill + `render_stone_wall()` pattern
- **Border**: `BorderTheme::Stone` ASCII art border

**DungeonTab** (`src/ui/components/dungeon/tab.rs`):
```rust
render_stone_wall(frame, area);
menu::render(frame, area, &mut self.list_state);
```

**DungeonScreen** (`src/ui/components/screens/dungeon.rs`):
```rust
// Fill background to match border
let bg_fill = Block::default().style(Style::default().bg(colors::MINE_BG));
frame.render_widget(bg_fill, area);
render_stone_wall(frame, area);
// ... content renders in inner_area (1px inset for border)
// Border rendered last using BorderTheme::Stone
```

Pattern follows other screens like `FightScreen` and `MineScreen` that render their own borders.

### Dungeon Header
The `render_header()` method in `DungeonScreen` displays only the dungeon name at the top of the screen. Located in `src/ui/components/screens/dungeon/mod.rs:350-370`.

## Minimap System

Located in `src/ui/components/dungeon/minimap.rs`. Renders in bottom-left corner of dungeon screen.

### Room Visibility States
- **Visited**: Player has entered the room (`is_visited = true`)
- **Revealed**: Room is adjacent to a visited room (`is_revealed = true`)
- **Hidden**: Not yet discovered (fog of war)

### Fog of War Behavior
- Entry room is visited on generation
- Adjacent rooms are revealed when player visits a room
- Revealed rooms stay visible permanently (don't re-fog)
- Empty spaces shown near revealed areas

### Minimap Icons (Nerdfont)
| Icon Code | Purpose | Display |
|-----------|---------|---------|
| `\u{f0787}` | Monster room | Crossed swords |
| `\u{eef8}` | Boss room | Dragon head |
| `\u{f0726}` | Chest room | Treasure chest |
| `\u{F023E}` | Rest room | Campfire |
| `\u{F0236}` | Trap room | Warning |
| `\u{F19D1}` | Treasure room | Gem |
| `\u{F415}` | Player location | Person |

### Color Coding
- Yellow: Current room
- Green: Cleared room
- Light grey: Visited but not cleared
- Dark grey: Revealed but not visited (shows `?`)

### Cell Format
Each room displayed as `[ X ]` (5 chars wide) for proper icon spacing.

## Rest Room Feature

### Overview
Rest rooms allow players to heal 50% of max HP once per room. They are pre-cleared (can leave and return, but cannot heal again).

### Key Files
| File | Purpose |
|------|---------|
| `src/ui/components/dungeon/campfire_art.rs` | Animated campfire ASCII art |
| `src/ui/components/screens/dungeon.rs` | `render_rest_room()` method |

### Rest Room UI Layout
```
[Animated Campfire]
    Rest Area
♥ [████████░░░░░░░░░░░░] 81/100

    > Rest (heal 50 HP)
      Continue
```

### Campfire Animation
Located in `src/ui/components/dungeon/campfire_art.rs`:
- 4-frame animation at 150ms per frame
- Uses `OnceLock<Instant>` for global animation timing
- 10 rows: sparks, flames (multiple layers), coals, logs, stone circle
- Width: 27 characters

**Transparent background rendering:**
```rust
// Skip spaces to preserve background
for span in line.spans {
    for ch in span.content.chars() {
        if ch != ' ' {
            buf.set_string(x, y, ch.to_string(), span.style);
        }
        x += 1;
    }
}
```

### HP Bar
Visual health display with color coding:
- Green: >60% HP
- Yellow: 30-60% HP
- Red: <30% HP

Format: `♥ [████████░░░░░░░░░░░░] current/max`

### State Handling
- `rest_selection: usize` tracks menu selection (0=Rest, 1=Continue)
- `DungeonRoom.has_healed: bool` tracks whether healing was used in this room
- `handle_rest_submit()` processes selection:
  - Rest: Heals 50% max HP using `player.heal(amount)`, sets `has_healed = true`
  - Continue: Transitions to `DungeonState::Navigation`
- When `has_healed` is true, shows "Already Rested" and disables healing option

## Boss Room Feature

### Overview
Each dungeon contains exactly one boss room with a dragon boss. The boss fight happens in-room (not in the normal FightScreen) and the player cannot escape until the boss is defeated.

### Key Files
| File | Purpose |
|------|---------|
| `src/dungeon/definition.rs` | `boss: Option<Mob>` field on Dungeon |
| `src/dungeon/generation.rs` | `place_boss_room()` places boss at dead ends |
| `src/ui/components/dungeon/dragon_art.rs` | Colored ASCII dragon art |
| `src/ui/components/screens/dungeon.rs` | `BossRoom` state and rendering |

### Boss Placement
- Spawns at dead ends (rooms with only 1 adjacent room)
- Excludes the starting room
- Falls back to any non-start room if no dead ends exist
- Boss room icon (`\u{eef8}` dragon head) revealed when adjacent (not `?`)

### Boss Storage
The dragon boss is stored on `Dungeon.boss: Option<Mob>`:
- Spawned once when first entering boss room via `gs.spawn_mob(MobId::Dragon)`
- Persists across renders (prevents HP fluctuation bug)
- Cleared when dungeon resets

### DungeonState::BossRoom
Added to the state machine:
```rust
enum DungeonState {
    RoomEntry,
    Navigation,
    RestRoom,
    BossRoom,  // In-room boss combat
}
```

### Boss Room UI Layout
```
[ASCII Dragon Art - 41x19 chars]

               DRAGON
♥ [████████░░░░░░░░░░░░] 81/100      (Boss HP - red/orange/yellow)
♥ [████████████████░░░░] 80/100  YOU (Player HP - green/yellow/red)

        [Combat Log]

         > Attack
```

- Dragon art rendered with multiple colors (scales, fire, eyes)
- Boss HP bar below dragon (ember red > flame orange > bright yellow color scheme)
- Player HP bar below boss HP (green > yellow > red color scheme)
- Combat log displays damage messages
- Single "Attack" option (no escape)

### In-Room Combat
`handle_submit()` in `boss_room.rs` implements boss combat without FightScreen:
1. Player attacks boss using `CombatSystem::attack()`
2. Log player damage dealt
3. If boss dies: reset dungeon and return to town (`gs.reset_dungeon()`, `gs.leave_dungeon()`)
4. If boss alive: boss counterattacks player
5. If player dies: kick out of dungeon, reset dungeon

### Dragon Removed from Mob Tables
Dragon is boss-only, removed from:
- `src/dungeon/traits.rs` - Dungeon mob_table
- `src/location/field/traits.rs` - Field mob weights
- `src/location/spec/specs.rs` - VILLAGE_FIELD spec

### Death Handling
Player death anywhere in dungeon (including boss):
- `FightScreen` checks for dungeon death: `gs.reset_dungeon()` + `gs.leave_dungeon()`
- Boss room death handled in `handle_boss_submit()` directly
- Shows toast: "You were defeated! Retreating from dungeon..."

### Fresh Dungeon Detection
In `view()`, detects new dungeon entry to reset screen state:
```rust
let visited_count = dungeon.rooms.iter().flatten()
    .filter(|r| r.as_ref().map(|room| room.is_visited).unwrap_or(false))
    .count();
let current_room_not_cleared = !current_room.is_cleared;

if visited_count == 1 && current_room_not_cleared && self.state != DungeonState::RoomEntry {
    self.state = DungeonState::RoomEntry;
    self.boss_combat_log.clear();
    self.reset_selection();
}
```
Note: Must check `current_room_not_cleared` to avoid resetting after clearing start room.

### Dragon ASCII Art
Located in `src/ui/components/dungeon/dragon_art.rs`:
- Dimensions: 41 wide x 19 tall
- Multi-colored using theme colors:
  - `scale_dark` / `scale` / `scale_light`: Forest greens for body
  - `eye`: Ember red for glowing eyes
  - `fire` / `fire_hot`: Orange/yellow for fire breath
  - `claw` / `teeth`: Light stone/white for claws and teeth
  - `inner`: Deep orange for inner glow

## Testing

Located in `src/dungeon/tests.rs`. Run with `cargo test dungeon::`.

### Test Categories (55 tests total)

| Category | Count | Description |
|----------|-------|-------------|
| DungeonRoom | 12 | Room creation, types, cleared states, visit/reveal |
| Direction | 6 | Offsets, names, `all()` method |
| Dungeon core | 20 | get_room, move_player, available_directions, counts |
| Generation | 12 | Room counts, required types, player start, connectivity |
| Integration | 5 | Player exploration, navigation walkthrough |

### Testing Notes

- **Global state dependency**: `open_chest()` calls `roll_drops()` which uses `game_state()`. Tests use `chest.take()` instead to avoid this.
- **Helper function**: `create_test_dungeon_with_rooms()` creates a simple 3-room L-shaped dungeon for testing.
- Generation tests verify guarantees: Chest room, Rest room, exactly 1 Boss room, Monster entry room.
