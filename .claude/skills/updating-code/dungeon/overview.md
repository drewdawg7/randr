# Dungeon System

## Overview
The dungeon system provides procedurally generated dungeon exploration with monster and chest rooms on a 5x5 grid.

## Key Files

| File | Purpose |
|------|---------|
| `src/dungeon/definition.rs` | Dungeon and DungeonRoom structs, core methods |
| `src/dungeon/generation.rs` | Procedural generation algorithm |
| `src/dungeon/enums.rs` | Direction, DungeonError, RoomType enums |
| `src/system.rs` | GameState dungeon field and CombatSource enum |
| `src/ui/components/dungeon/tab.rs` | Town tab entry point (DungeonTab) |
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
}
```

**State transitions:**
- `RoomEntry` -> `Navigation`: When `room.is_cleared && state == RoomEntry` (checked in `view()`)
- `Navigation` -> `RoomEntry`: After `move_player()` to new room

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
2. Random walk to create 9-15 contiguous rooms (60% max fill)
3. Room types: 70% Monster, 30% Chest
4. Entry room is always Monster type

## Key Methods

### Dungeon
- `generate()` - Creates room layout
- `spawn_mob()` - Weighted random mob from `mob_table`
- `move_player(Direction)` - Update player position
- `available_directions()` - Get valid movement options
- `current_room()` / `current_room_mut()` - Get room at player position

### DungeonRoom
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
