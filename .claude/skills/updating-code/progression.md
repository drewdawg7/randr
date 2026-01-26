# Progression System

Player leveling and XP mechanics.

**File:** `src/entities/progression.rs`

## Core Struct

```rust
#[derive(Resource, Debug, Default, Clone)]
pub struct Progression {
    pub level: i32,
    pub xp: i32,       // Current XP toward next level
    pub total_xp: i32  // Lifetime XP earned
}
```

## XP Curve

Uses **10% compound growth** with base 50 XP:

```rust
pub fn xp_to_next_level(level: i32) -> i32 {
    (50.0 * 1.1_f64.powi(level - 1)).round() as i32
}
```

| Level | XP Required |
|-------|-------------|
| 1→2   | 50          |
| 2→3   | 55          |
| 3→4   | 61          |
| 4→5   | 67          |
| 5→6   | 73          |
| 10→11 | 118         |

## Key Methods

- `Progression::new()` — Starts at level 1, 0 XP
- `Progression::add_xp(xp)` — Adds XP, handles level-ups, returns levels gained
- `Progression::xp_to_next_level(level)` — Static function for XP curve

## HasProgression Trait

Allows any entity to have progression:

```rust
pub trait HasProgression {
    fn progression(&self) -> &Progression;
    fn progression_mut(&mut self) -> &mut Progression;
    fn level(&self) -> i32;
    fn on_level_up(&mut self);  // Called once per level gained
    fn gain_xp(&mut self, amount: i32) -> i32;  // Returns levels gained
}
```

## GivesXP Trait

For entities that award XP when defeated:

```rust
pub trait GivesXP {
    fn give_xp(&self) -> i32;
}
```

## UI Display

XP is displayed in the player stats banner — see [player-stats.md](player-stats.md).

Format: `"Level: {lvl}  XP: {xp}/{needed}"`
