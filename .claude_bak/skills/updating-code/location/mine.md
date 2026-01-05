# Mine System Overview

## Module Structure

```
src/location/mine/
├── mod.rs           # Exports: Mine, CaveLayout, CaveRock, RockType, etc.
├── definition.rs    # Mine struct with cave and timers
├── cave.rs          # CaveLayout procedural generation and state
├── traits.rs        # Location trait impl, Default impl
├── tests.rs         # Unit tests
└── rock/            # Rock submodule (RockId, RockRegistry, etc.)
```

## Key Types

### Mine (`definition.rs`)
The main mine location struct:
```rust
pub struct Mine {
    pub location_id: LocationId,
    pub name: String,
    pub rock_weights: HashMap<RockId, i32>,
    pub current_rock: Option<Rock>,
    pub cave: Option<CaveLayout>,        // Persistent cave state
    last_rock_respawn: Instant,          // 2-minute timer
    last_regeneration: Instant,          // 10-minute timer
}
```

### CaveLayout (`cave.rs`)
Procedurally generated cave with cellular automata:
- `cells: [[Cell; 60][20]]` - Wall/Floor grid
- `rocks: Vec<CaveRock>` - Positioned rocks (max 8)
- `player_x/y` - Player position
- `exit_x/y` - Exit ladder position

Key methods:
- `generate()` - Creates new cave with cellular automata
- `spawn_rock()` - Add rock at random floor position
- `move_player(dx, dy)` - Handle player movement
- `mine_adjacent_rock()` - Remove and return adjacent rock
- `is_on_exit()` - Check if player on ladder

### RockType (`cave.rs`)
```rust
pub enum RockType { Copper, Coal, Tin }
```
Each has color and loot table. Spawn weights: Copper 50%, Coal 30%, Tin 20%.

## Timer System

### Constants (`definition.rs`)
- `ROCK_RESPAWN_INTERVAL`: 2 minutes - spawns new rock if < 8
- `MINE_REGENERATION_INTERVAL`: 10 minutes - regenerates entire cave

### Methods
- `check_and_respawn_rock()` - Called in game loop, spawns rock if interval elapsed
- `check_and_regenerate()` - Called in game loop, regenerates cave if interval elapsed
- `time_until_regeneration()` - Returns seconds until next regeneration (for UI)
- `time_until_rock_respawn()` - Returns seconds until next rock spawn

### Game Loop Integration (`system.rs`)
```rust
pub fn run_current_screen(&mut self) {
    self.town.store.check_and_restock();
    self.town.mine.check_and_regenerate();  // Check regeneration
    self.town.mine.check_and_respawn_rock(); // Check rock respawn
    // ...
}
```

## UI Integration

### Field Menu (`ui/components/field/menu.rs`)
Shows mine regeneration timer next to Mine option:
```
⛏ Mine (X:XX)
```
Timer displays minutes:seconds until next mine regeneration.

### MineScreen (`ui/components/mine/screen.rs`)
- Uses persistent cave from `game_state().town.mine.cave`
- No longer regenerates cave on screen entry
- Player can move with arrow keys, mine with Space, exit with Backspace

### Cave Rendering (`ui/components/mine/cave_art.rs`)
Pure rendering module - takes `&CaveLayout` and renders:
- Procedural cave walls with varying characters (#, @, %, ;)
- Colored rocks (Copper/Coal/Tin)
- Player icon with pickaxe indicator when adjacent to rock
- Exit ladder with arrow indicator when standing on it

## Files to Modify

| Change | Files |
|--------|-------|
| Adjust spawn/regen timing | `mine/definition.rs` (constants) |
| Add new rock types | `mine/cave.rs` (RockType), rock specs |
| Change cave size | `mine/cave.rs` (CAVE_WIDTH/HEIGHT) |
| Modify timer display | `ui/components/field/menu.rs` |
| Change cave generation | `mine/cave.rs` (CaveLayout::generate) |
